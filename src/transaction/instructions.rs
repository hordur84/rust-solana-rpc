use pyo3::prelude::*;
use serde::{Serialize, Deserialize};

/// Solana Transfer system instruction construct.
/// Contains `lamports` field.
#[derive(Serialize, Deserialize)]
pub struct InstructionTransfer {
    /// Number of lamports to transfer.
    pub lamports: u64
}

/// Wrapper for a decoded transfer system instruction with python bindings.
/// Contains `amount`, `source`, `destination`, `action`, `block_time`, `human_time` and `signature`.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct InstructionTransferWrapper {
    /// Program account that executed the instruction.
    #[pyo3(get)]
    pub program: String,
    /// Amount being transferred from `source` to `destination` account.
    #[pyo3(get)]
    pub amount: f64,
    /// Source account.
    #[pyo3(get)]
    pub source: String,
    /// Destination account.
    #[pyo3(get)]
    pub destination: String,
    /// The instruction performed.
    #[pyo3(get)]
    pub action: String,
    /// Estimated production time, as UNIX timestamp.
    #[pyo3(get)]
    pub block_time: u64,
    /// Estimated production time, as human readable formatted string.
    #[pyo3(get)]
    pub human_time: String,
    /// Transaction signature.
    #[pyo3(get)]
    pub signature: String
}