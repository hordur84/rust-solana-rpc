use pyo3::prelude::*;
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

/// Get SOL transactions.
#[pyfunction]
fn get_transactions(account: String, start: String, end: String, depth: i32) -> PyResult<Vec<InstructionTransferWrapper>> {

    /* Get the RPC client */
    let client_config = ClientConfig {
        url: "https://ssc-dao.genesysgo.net/".to_string(),
        timeout: Duration::from_secs(20),
        commitment: CommitmentConfig::finalized()
    };

    let client = get_client(client_config);

    /* Specify signature configuration */
    let signature_config = SignatureConfig {
        time_before: Some(start),
        time_after: Some(end),
        before: None,
        until: None,
        limit: Some(1000),
        commitment: Some(CommitmentConfig::finalized())
    };

    /* Get SOL transfers */
    let mut data: Vec<InstructionTransferWrapper> = vec![];
    get_transfers_recursive(&account, &client, &signature_config, depth, &mut data);

    Ok(data.to_vec())

}

/// A Python module implemented in Rust.
#[pymodule]
fn solana_rpc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_transactions, m)?)?;
    Ok(())
}