use solana_client::{rpc_client::RpcClient, client_error::ClientError};
use solana_sdk::pubkey::Pubkey;
use core::result;
use std::io;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

/**
 * Function to return SOL balance from a pubkey.
 */
pub fn get_balance(client: &RpcClient, pubkey: &Pubkey) -> result::Result<f64, ClientError> {

    let balance = client.get_balance(pubkey)?;
    Ok(balance as f64/LAMPORTS_PER_SOL as f64)
}

/**
 * Function to print out SOL balance from a pubkey.
 */
pub fn show_balance(client: &RpcClient, pubkey: &Pubkey) -> io::Result<()> {

    let balance = client.get_balance(pubkey).unwrap();
    println!("Balance for address: {} - is: {}", pubkey, balance);
    Ok(())
}