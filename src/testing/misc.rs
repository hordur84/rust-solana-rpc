use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::UiTransactionEncoding;
use solana_sdk::signature::Signature;
use serde_json;

/**
 * Display raw transaction from a signature.
 */
pub fn show_transaction_raw(client: &RpcClient, signature: &str) {

    let signature = Signature::from_str(signature).unwrap();
    let tx = client.get_transaction(&signature, UiTransactionEncoding::Json).unwrap();

    println!("Transaction details for signature: {}\n {}", signature, serde_json::to_string_pretty(&tx).unwrap());
}