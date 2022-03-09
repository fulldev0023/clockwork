use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use cronos_sdk::account::*;
use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn health_start(client: &Arc<Client>) -> Result<(), CliError> {
    // Derive PDAs
    let admin = client.payer_pubkey();
    let authority = Authority::pda().0;
    let config = Config::pda().0;
    let daemon = Daemon::pda(authority).0;
    let health = Health::pda().0;

    // Fetch daemon data
    let data = client.get_account_data(&daemon).unwrap();
    let daemon_data = Daemon::try_from(data).unwrap();

    // Fetch config data
    let data = client.get_account_data(&config).unwrap();
    let config_data = Config::try_from(data).unwrap();

    // Task PDA
    let task_pda = Task::pda(daemon, daemon_data.task_count);

    // Build instructions
    let schedule = String::from("10 * * * *"); // Every 10 seconds
    let ix_a = cronos_sdk::instruction::admin_reset_health(admin, config, health);
    let health_ping_ix = cronos_sdk::instruction::health_ping(authority, config, daemon, health);
    let ix_b = cronos_sdk::instruction::admin_create_task(
        task_pda,
        admin,
        authority,
        config,
        daemon,
        health_ping_ix,
        schedule,
    );

    // Sign and submit
    sign_and_submit(client, &[ix_a, ix_b]);
    Ok(())
}
