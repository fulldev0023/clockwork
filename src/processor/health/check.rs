use std::sync::Arc;

use solana_client_helpers::Client;

use crate::error::CliError;

pub fn check(client: &Arc<Client>) -> Result<(), CliError> {
    // let owner = client.payer_pubkey();
    let health_addr = cronos_sdk::account::Health::find_pda().0;
    let data = client
        .get_account_data(&health_addr)
        .map_err(|_err| CliError::AccountNotFound(health_addr.to_string()))?;
    let health_data = cronos_sdk::account::Health::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(health_addr.to_string()))?;
    println!("{}", health_data);
    Ok(())
}
