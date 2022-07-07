use solana_sdk::commitment_config::CommitmentConfig;
use std::time::Duration;

pub mod parsing;
pub mod transaction;
pub mod hash;
pub mod errors;
pub mod constants;
pub mod client;
pub mod signature;
pub mod logic;

use client::{ClientConfig, get_client};
use signature::signature::SignatureConfig;
use logic::get_transfers_recursive;
use transaction::instructions::InstructionTransferWrapper;

fn main() {

    /* Get the RPC client */
    let client_config = ClientConfig {
        url: "https://ssc-dao.genesysgo.net/".to_string(),
        timeout: Duration::from_secs(20),
        commitment: CommitmentConfig::finalized()
    };

    let client = get_client(client_config);

    /* Specify account */
    let account = "dDE2MCJ777CCfY4ytUzuD7RFiGnvJJrHrZ3vzeTZESo".to_string();

    /* Specify signature configuration */
    let signature_config = SignatureConfig {
        time_before: Some("2022-05-20 12:00:00".to_string()),
        time_after: Some("2022-05-26 19:58:00".to_string()),
        before: None,
        until: None,
        limit: Some(1000),
        commitment: Some(CommitmentConfig::finalized())
    };

    /* Get SOL transfers */
    let mut data: Vec<InstructionTransferWrapper> = vec![];
    get_transfers_recursive(&account, &client, &signature_config, 2, &mut data);

}
