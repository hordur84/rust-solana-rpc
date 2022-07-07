use base58::FromBase58;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::constants::SYSTEM_PROGRAM; 
use crate::parsing::time::convert_unix_to_time;
use super::transaction::InstructionProcessed;
use super::instructions::{InstructionTransfer, InstructionTransferWrapper};
use super::enums::SystemProgramInstruction;

/// Decode inner instruction from transaction and wraps it in an `InstructionTransferWrapper`.
/// * TODO: In the future it should return an `Option<TraitObject>` for extendability.
pub fn decode_instruction(ix: InstructionProcessed) -> Option<InstructionTransferWrapper> {

    let account_program = &ix.executer[..];

    let data = match ix.data.from_base58() {
        Ok(data) => Some(data),
        Err(_) => return None
    };

    let data = data.unwrap();

    match account_program {
        SYSTEM_PROGRAM => {
            let action: SystemProgramInstruction = bincode::deserialize(&data[..4]).unwrap();
            match action {
                // SystemProgram transfer instruction.
                // Account references: 
                // [0] => funding account.
                // [1] => recipient account.
                SystemProgramInstruction::Transfer => {
                    let data: InstructionTransfer = bincode::deserialize(&data[4..]).unwrap();
                    let data = InstructionTransferWrapper {
                        amount: data.lamports as f64 / LAMPORTS_PER_SOL as f64,
                        program: account_program.to_string(),
                        source: ix.accounts[0].clone(),
                        destination: ix.accounts[1].clone(),
                        action: SystemProgramInstruction::Transfer.to_string(),
                        block_time: ix.block_time,
                        human_time: convert_unix_to_time(ix.block_time),
                        signature: ix.signature,
                    };
                    Some(data)
                },
                _ => None
            }
        },
        _ => None
    }
}