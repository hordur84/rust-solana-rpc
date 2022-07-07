use serde::{Deserialize, Serialize};
use std::fmt;

/// Solana SystemInstruction enum as per specifications:
/// - https://github.com/solana-labs/solana/blob/6606590b8132e56dab9e60b3f7d20ba7412a736c/sdk/program/src/system_instruction.rs
/// - The first 4 bytes of a SystemInstruction point to the type of the instruction.
#[derive(Debug, Deserialize, Serialize)]
pub enum SystemProgramInstruction {
    CreateAccount,
    Assign,
    Transfer,
    CreateAccountWithSeed,
    AdvancedNonceAccount,
    WithdrawNonceAccount,
    InitializeNonceAccount,
    AuthorizeNonceAccount,
    Allocate,
    AllocateWithSeed,
    AssignWithSeed,
    TransferWithSeed
}

impl fmt::Display for SystemProgramInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

