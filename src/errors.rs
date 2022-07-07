use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaSignatureError {
    #[error("Failed to parse pubkey address for signature")]
    SignatureParsePubkeyError(String),
}

#[derive(Error, Debug)]
pub enum TransactionDetailError {
    #[error("Failed to parse signature for transaction")]
    ParseSignatureError(String),
    #[error("Failed to get transaction")]
    ClientError(String),
    #[error("Failed to parse transaction")]
    TransactionParseError(String),
    #[error("Failed to serialize transaction")]
    SerializeError(String),
    #[error("Failed to deserialize transaction")]
    DeserializeError(String),
    #[error("Failed to get transaction transaction data")]
    TransactionDataError(String),
    #[error("Failed to get transaction meta data")]
    TransactionMetaError(String),
    #[error("Failed to get inner instruction")]
    TransactionInnerInstruction(String),
    #[error("Failed to get instructions")]
    TransactionInstruction(String)
}