use solana_client::rpc_client::RpcClient;

use crate::signature::signature::{SignatureConfig, Signatures};
use crate::transaction::decode::decode_instruction;
use crate::transaction::instructions::InstructionTransferWrapper;
use crate::transaction::transaction::TransactionDetail;

/// Get SOL transfers starting from `account`.
/// 
/// # Arguments
/// * `account` - Account as base-58 encoded string.
/// * `client` - RPC client.
/// * `config` - Config for signature query.
/// * `breadth` - Depth of signature query.
/// * `finished_signatures` - Array of `SData` hashes to keep track of seen instructions.
/// ```
pub fn get_transfers_recursive(account: &String, client: &RpcClient, config: &SignatureConfig, breadth: i32, data: &mut Vec<InstructionTransferWrapper>) -> Vec<InstructionTransferWrapper> {
    if breadth < 0 { return data.to_vec() };
    let signatures = Signatures::fetch(&account, &client, &config).unwrap();
    for signature in &signatures {
        let transaction = TransactionDetail::new(&signature.signature, &client);
        let intsructions = transaction.get_instructions_processed();
        for inner_instruction in intsructions {
            if let Some(decoded) = decode_instruction(inner_instruction) {
                if data.contains(&decoded) {
                    continue;
                }
                else {
                    println!("Decoded branch:\n{:?}", decoded);
                    let destination = &decoded.destination.clone();
                    data.push(decoded);
                    get_transfers_recursive(destination, &client, &config, breadth-1, data);
                }
            }
        }
    }
    data.to_vec()
}

/// Display debug information to output.
/// 
/// # Arguments
/// 
/// * `account` - Account as base-58 encoded string.
/// * `client` - RCP client.
/// * `config` - Config for Signature query.
/// * `signature_index` Index into the signatures array.
/// ```
pub fn display_debug(account: &String, client: &RpcClient, config: &SignatureConfig, signature_index: usize) {

    /* Get signatures for the specified configuration */
    let signatures = Signatures::fetch(&account, &client, &config).unwrap();
    for signature in &signatures {
        println!("{}", signature);
    }

    /* Get a transaction for a specified signature */
    let transaction = TransactionDetail::new(&signatures[signature_index].signature, &client);
    transaction.print();

    /* Decode supported instructions for transaction. */
    let instructions = transaction.get_instructions_processed();
    for instruction in instructions {
        if let Some(decoded) = decode_instruction(instruction) {
            println!("Decoded\n{:?}", decoded);
        }
    }
}