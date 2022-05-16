use {
    crate::{
        cli::CliError, parser::JsonInstructionData, utils::new_client, utils::sign_and_submit,
    },
    chrono::{prelude::*, Duration},
    cronos_cron::Schedule,
    cronos_sdk::scheduler::events::QueueExecuted,
    cronos_sdk::scheduler::state::{Fee, Queue, Task, Yogi},
    serde_json::json,
    solana_client::{
        pubsub_client::PubsubClient,
        rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    },
    solana_client_helpers::Client,
    solana_sdk::{
        borsh, commitment_config::CommitmentConfig, instruction::Instruction,
        native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    },
    std::{collections::HashMap, ops::Div, str::FromStr, sync::Arc},
};

pub fn run(count: u32, parallelism: f32, recurrence: u32) -> Result<(), CliError> {
    // Setup test
    let client = new_client();
    let num_queues_parallel = (count as f32 * parallelism) as u32;
    let num_queues_serial = count - num_queues_parallel;

    let mut owners: Vec<Keypair> = vec![];
    let mut expected_exec_ats = HashMap::<Pubkey, Vec<i64>>::new();
    let mut actual_exec_ats = HashMap::<Pubkey, Vec<i64>>::new();

    // Create yogis
    for _ in 0..(num_queues_parallel + 1) {
        let owner = Keypair::new();
        let yogi_pubkey = Yogi::pda(owner.pubkey()).0;
        let fee_pubkey = Fee::pda(yogi_pubkey).0;
        let ix = cronos_sdk::scheduler::instruction::yogi_new(
            fee_pubkey,
            owner.pubkey(),
            owner.pubkey(),
            yogi_pubkey,
        );
        client.airdrop(&owner.pubkey(), LAMPORTS_PER_SOL).unwrap();
        sign_and_submit(&client, &[ix], &owner);
        owners.push(owner);
    }

    // Schedule parallel queues
    for i in 0..num_queues_parallel {
        let owner = owners.get(i as usize).unwrap();
        schedule_memo_queue(&client, owner, recurrence, &mut expected_exec_ats);
    }

    // Schedule serial queues
    let owner = owners.last().unwrap();
    for _ in 0..num_queues_serial {
        schedule_memo_queue(&client, owner, recurrence, &mut expected_exec_ats);
    }

    // Collect and report test results
    let num_expected_events = count * (recurrence + 1);
    listen_for_events(num_expected_events, &mut actual_exec_ats)?;
    calculate_and_report_stats(num_expected_events, expected_exec_ats, actual_exec_ats)?;

    Ok(())
}

fn listen_for_events(
    num_expected_events: u32,
    actual_exec_ats: &mut HashMap<Pubkey, Vec<i64>>,
) -> Result<(), CliError> {
    let (ws_sub, log_receiver) = PubsubClient::logs_subscribe(
        "ws://localhost:8900/",
        RpcTransactionLogsFilter::Mentions(vec![cronos_sdk::scheduler::ID.to_string()]),
        RpcTransactionLogsConfig {
            commitment: Some(CommitmentConfig::confirmed()),
        },
    )
    .map_err(|_| CliError::WebsocketError)?;

    // Watch for queue exec events
    let mut event_count = 0;

    for log_response in log_receiver {
        let logs = log_response.value.logs.into_iter();
        for log in logs {
            match &log[..14] {
                "Program data: " => {
                    // Decode event from log data
                    let mut buffer = Vec::<u8>::new();
                    base64::decode_config_buf(&log[14..], base64::STANDARD, &mut buffer).unwrap();
                    let event =
                        borsh::try_from_slice_unchecked::<QueueExecuted>(&buffer[8..]).unwrap();
                    actual_exec_ats
                        .entry(event.queue)
                        .or_insert(Vec::new())
                        .push(event.ts);
                    event_count += 1;
                }
                _ => {}
            }
        }

        // Exit if we've received the expected number of events
        if event_count == num_expected_events {
            break;
        }
    }

    // TODO: Remove once https://github.com/solana-labs/solana/issues/16102
    //       is addressed. Until then, drop the subscription handle in another
    //       thread so that we deadlock in the other thread as to not block
    //       this thread.
    std::thread::spawn(move || {
        ws_sub.send_unsubscribe().unwrap();
    });

    Ok(())
}

fn build_memo_ix(yogi_pubkey: &Pubkey) -> Instruction {
    let hello_world_memo = json!({
      "program_id": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
      "accounts": [
        {
          "pubkey": yogi_pubkey.to_string(),
          "is_signer": true,
          "is_writable": false
        }
      ],
      "data": [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    });
    Instruction::try_from(
        &serde_json::from_value::<JsonInstructionData>(hello_world_memo)
            .expect("JSON was not well-formatted"),
    )
    .unwrap()
}

fn schedule_memo_queue(
    client: &Arc<Client>,
    owner: &Keypair,
    recurrence: u32,
    expected_exec: &mut HashMap<Pubkey, Vec<i64>>,
) {
    // Get yogi for owner
    let yogi_pubkey = Yogi::pda(owner.pubkey()).0;
    let yogi = client
        .get_account_data(&yogi_pubkey)
        .map_err(|_err| CliError::AccountNotFound(yogi_pubkey.to_string()))
        .unwrap();
    let yogi_data = Yogi::try_from(yogi)
        .map_err(|_err| CliError::AccountDataNotParsable(yogi_pubkey.to_string()))
        .unwrap();

    // Generate PDA for new queue account
    let queue_pubkey = Queue::pda(yogi_pubkey, yogi_data.queue_count).0;
    let now: DateTime<Utc> = Utc::now();
    let next_minute = now + Duration::minutes(1);
    let schedule = format!(
        "0-{} {} {} {} {} {} {}",
        recurrence,
        next_minute.minute(),
        next_minute.hour(),
        next_minute.day(),
        next_minute.month(),
        next_minute.weekday(),
        next_minute.year()
    );
    let create_queue_ix = cronos_sdk::scheduler::instruction::queue_new(
        owner.pubkey(),
        owner.pubkey(),
        yogi_pubkey,
        schedule.clone(),
        queue_pubkey,
    );

    // Index expected exec_at times
    for datetime in Schedule::from_str(&schedule)
        .unwrap()
        .after(&Utc.from_utc_datetime(&Utc::now().naive_utc()))
    {
        expected_exec
            .entry(queue_pubkey)
            .or_insert(Vec::new())
            .push(datetime.timestamp());
    }

    // Create an task
    let task_pubkey = Task::pda(queue_pubkey, 0).0;
    let memo_ix = build_memo_ix(&yogi_pubkey);
    let create_task_ix = cronos_sdk::scheduler::instruction::task_new(
        task_pubkey,
        vec![memo_ix],
        owner.pubkey(),
        owner.pubkey(),
        yogi_pubkey,
        queue_pubkey,
    );

    sign_and_submit(&client, &[create_queue_ix, create_task_ix], owner);
}

fn calculate_and_report_stats(
    num_expected_events: u32,
    expecteds: HashMap<Pubkey, Vec<i64>>,
    actuals: HashMap<Pubkey, Vec<i64>>,
) -> Result<(), CliError> {
    // Calculate delays
    let mut delays: Vec<i64> = vec![];
    let mut missing = 0;
    for (queue_pubkey, expecteds) in expecteds {
        for i in 0..expecteds.len() {
            let expected = expecteds.get(i).unwrap();
            let actual = actuals.get(&queue_pubkey).unwrap().get(i);
            match actual {
                None => missing += 1,
                Some(actual) => {
                    delays.push(actual - expected);
                }
            }
        }
    }
    delays.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Compute stats on delay data
    let mean = delays.iter().sum::<i64>() as f32 / delays.len() as f32;
    let mid = delays.len() / 2;
    let std_dev = delays
        .iter()
        .map(|value| {
            let diff = mean - (*value as f32);
            diff * diff
        })
        .sum::<f32>()
        .div(delays.len() as f32)
        .sqrt();

    // Stdout
    println!("Expected execs: {}", num_expected_events);
    println!("Missing execs: {}", missing);
    println!("Delay (mean): {} sec", mean);
    println!("Delay (median): {} sec", delays[mid]);
    println!("Delay (stddev): {} sec", std_dev);

    Ok(())
}
