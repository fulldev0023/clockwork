use {
    crate::{client::RPCClient, Bucket, Config, Filter, TaskCache},
    bincode::deserialize,
    cronos_sdk::account::{Fee, Task, TaskStatus},
    log::{debug, info},
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPlugin, AccountsDbPluginError as PluginError, ReplicaAccountInfo,
        ReplicaAccountInfoVersions, Result as PluginResult,
    },
    solana_client_helpers::Client,
    solana_program::{clock::Clock, pubkey::Pubkey, sysvar},
    solana_sdk::instruction::AccountMeta,
    std::{
        fmt::{Debug, Formatter},
        sync::Mutex,
        sync::{Arc, RwLock},
        thread::{self, JoinHandle},
    },
    thiserror::Error,
};

#[derive(Clone)]
pub struct CronosPlugin {
    client: Option<Arc<Client>>,
    cache: Option<Arc<RwLock<TaskCache>>>,
    bucket: Option<Arc<Mutex<Bucket>>>,
    filter: Option<Filter>,
    latest_clock_value: i64,
}

impl Debug for CronosPlugin {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CronosPluginError {
    #[error("Error reading and/or writing to local cache. Error message: ({msg})")]
    CacheError { msg: String },

    #[error("Error deserializing task data")]
    TaskAccountInfoError,

    #[error("Error deserializing sysvar clock data")]
    ClockAccountInfoError,
}

impl AccountsDbPlugin for CronosPlugin {
    fn name(&self) -> &'static str {
        "CronosPlugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");

        info!("Loading plugin {:?}", self.name());
        info!("Program ID: {}", &cronos_sdk::ID);
        info!("config_file: {:?} ", config_file);

        let result = Config::read_from(config_file);

        match result {
            Err(err) => {
                return Err(PluginError::ConfigFileReadError {
                    msg: format!(
                        "The config file is not in the JSON format expected: {:?}",
                        err
                    ),
                })
            }
            Ok(config) => self.filter = Some(Filter::new(&config)),
        }

        self.bucket = Some(Arc::new(Mutex::new(Bucket::new())));
        self.cache = Some(Arc::new(RwLock::new(TaskCache::new())));
        self.client = Some(Arc::new(Client::new()));
        self.latest_clock_value = 0;

        info!("Loaded Cronos Plugin");

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Unloading plugin: {:?}", self.name());

        self.bucket = None;
        self.cache = None;
        self.client = None;
        self.filter = None;
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        if is_startup {
            return Ok(());
        }

        let info = Self::unwrap_update_account(account);

        if !self.unwrap_filter().wants_program(info.owner) {
            return Ok(());
        }

        debug!(
            "Updating account {:?} with owner {:?} at slot {:?}",
            info.pubkey, info.owner, slot
        );

        match &mut self.cache {
            None => {
                return Err(PluginError::Custom(Box::new(
                    CronosPluginError::CacheError {
                        msg: "There is no available cache to update account data".to_string(),
                    },
                )));
            }
            Some(_cache) => {
                if &sysvar::clock::id().to_bytes() == info.pubkey {
                    let clock = deserialize::<Clock>(info.data);

                    match clock {
                        Err(_err) => {
                            return Err(PluginError::Custom(Box::new(
                                CronosPluginError::ClockAccountInfoError,
                            )))
                        }
                        Ok(clock) => {
                            if self.latest_clock_value < clock.unix_timestamp {
                                self.latest_clock_value = clock.unix_timestamp;
                                self.execute_tasks_in_lookback_window();
                            }
                        }
                    }
                } else if &cronos_sdk::ID.to_bytes() == info.owner {
                    let task = Task::try_from(info.data.to_vec());
                    let key = Pubkey::new(info.pubkey);

                    match task {
                        Err(_err) => {
                            return Err(PluginError::Custom(Box::new(
                                CronosPluginError::TaskAccountInfoError,
                            )))
                        }
                        Ok(task) => {
                            self.replicate_task(key, task);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> PluginResult<()> {
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        _slot: u64,
        _parent: Option<u64>,
        _status: solana_accountsdb_plugin_interface::accountsdb_plugin_interface::SlotStatus,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        _transaction: solana_accountsdb_plugin_interface::accountsdb_plugin_interface::ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_block_metadata(
        &mut self,
        _blockinfo: solana_accountsdb_plugin_interface::accountsdb_plugin_interface::ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }
}

impl CronosPlugin {
    pub fn new() -> Self {
        Self {
            client: Some(Arc::new(Client::new())),
            cache: Some(Arc::new(RwLock::new(TaskCache::new()))),
            bucket: Some(Arc::new(Mutex::new(Bucket::new()))),
            filter: None,
            latest_clock_value: 0,
        }
    }
    fn unwrap_bucket(&self) -> &Arc<Mutex<Bucket>> {
        self.bucket.as_ref().expect("client is unavailable")
    }
    fn unwrap_cache(&self) -> &Arc<RwLock<TaskCache>> {
        self.cache.as_ref().expect("cache is unavailable")
    }
    fn unwrap_client(&self) -> &Arc<Client> {
        self.client.as_ref().expect("client is unavailable")
    }
    fn unwrap_filter(&self) -> &Filter {
        self.filter.as_ref().expect("filter is unavailable")
    }
    fn unwrap_update_account(account: ReplicaAccountInfoVersions) -> &ReplicaAccountInfo {
        match account {
            ReplicaAccountInfoVersions::V0_0_1(info) => info,
        }
    }

    fn replicate_task(&self, key: Pubkey, task: Task) {
        info!("💽 Replicating task {}", key);
        let mut w_cache = self.unwrap_cache().write().unwrap();
        match task.status {
            TaskStatus::Queued => w_cache.insert(key, task),
            TaskStatus::Cancelled | TaskStatus::Done => w_cache.delete(key),
        }
    }

    fn execute_tasks_in_lookback_window(&self) {
        let self_clone = self.clone();
        let cp_arc: Arc<CronosPlugin> = Arc::new(self_clone);
        let cp_clone = cp_arc.clone();

        thread::spawn(move || {
            const LOOKBACK_WINDOW: i64 = 60 * 15; // Number of seconds to lookback
            info!(
                "executing tasks for unix_ts: {}",
                cp_clone.latest_clock_value
            );

            // Spawn threads to execute tasks in lookback window
            let mut handles = vec![];
            for t in (cp_clone.latest_clock_value - LOOKBACK_WINDOW)..=cp_clone.latest_clock_value {
                let r_cache = cp_clone.unwrap_cache().read().unwrap();
                r_cache.index.get(&t).and_then(|keys| {
                    for key in keys.iter() {
                        r_cache.data.get(key).and_then(|task| {
                            handles.push(execute_task(
                                cp_clone.unwrap_client().clone(),
                                cp_clone.unwrap_cache().clone(),
                                cp_clone.unwrap_bucket().clone(),
                                *key,
                                task.clone(),
                            ));
                            Some(())
                        });
                    }
                    Some(())
                });
            }

            // Join threads
            if !handles.is_empty() {
                for h in handles {
                    h.join().unwrap();
                }
            }
        });
    }
}

fn execute_task(
    client: Arc<Client>,
    cache: Arc<RwLock<TaskCache>>,
    bucket: Arc<Mutex<Bucket>>,
    key: Pubkey,
    task: Task,
) -> JoinHandle<()> {
    thread::spawn(move || {
        // Lock the mutex for this task
        let mutex = bucket
            .lock()
            .unwrap()
            .get_mutex((key, task.schedule.exec_at));
        let guard = mutex.try_lock();
        if guard.is_err() {
            return;
        };
        let guard = guard.unwrap();

        // Get accounts
        let config = cronos_sdk::account::Config::pda().0;
        let fee = Fee::pda(task.daemon).0;

        // Add accounts to exec instruction
        let mut ix_exec = cronos_sdk::instruction::task_execute(
            config,
            task.daemon,
            fee,
            key,
            client.payer_pubkey(),
        );
        for acc in task.ix.accounts {
            match acc.is_writable {
                true => ix_exec.accounts.push(AccountMeta::new(acc.pubkey, false)),
                false => ix_exec
                    .accounts
                    .push(AccountMeta::new_readonly(acc.pubkey, false)),
            }
        }
        ix_exec
            .accounts
            .push(AccountMeta::new_readonly(task.ix.program_id, false));

        // Sign and submit
        let res = client.sign_and_submit(
            &[ix_exec],
            format!("🤖 Executing task: {} {}", key, task.schedule.exec_at).as_str(),
        );

        // If exec failed, replicate the task data
        if res.is_err() {
            let err = res.err().unwrap();
            info!("❌ {}", err);
            let data = client.get_account_data(&key).unwrap();
            let task = Task::try_from(data).unwrap();
            let mut w_cache = cache.write().unwrap();
            w_cache.insert(key, task);
        }

        // Drop the mutex
        drop(guard)
    })
}
