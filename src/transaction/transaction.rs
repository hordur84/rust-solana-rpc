use std::str::FromStr;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use crate::errors::TransactionDetailError;

impl TransactionDetail {

    /// Create `TransactionDetail` object. Wrapper for a processed transaction's data.
    /// 
    /// # Arguments
    /// 
    /// * `signature` - Transaction signature encoded as a baes-58 string slice.
    /// * `client` - RPC client.
    /// ```
    pub fn new(signature: &str, client: &RpcClient) -> Self {

        let tx = Self::process_transaction(signature, client).unwrap();
        tx
    }

    /// Print raw transaction to output.
    pub fn print(&self) {
        println!("{}", serde_json::to_string_pretty(&self).unwrap());
    }

    /// Return an array of base-58 encoded public keys used by the transaction.
    pub fn get_account_keys(&self) -> Result<&Vec<String>, TransactionDetailError> {
        match &self.transaction {
            Some(tx) => Ok(&tx.message.account_keys),
            None => return Err(TransactionDetailError::TransactionDataError("Failed to get account keys.".to_string()))
        }
    }

    /// Return transaction signature id as base-58 encoded string.
    pub fn get_transaction_signature_id(&self) -> Result<&String, TransactionDetailError> {
        match &self.transaction {
            Some(signature) => Ok(&signature.signatures[0]),
            None => return Err(TransactionDetailError::TransactionDataError("Failed to get transaction signature".to_string()))
        }
    }

    /// Return time when transaction was processed as UNIX timestamp.
    pub fn get_block_time(&self) -> Result<&u64, TransactionDetailError> {
        match &self.block_time {
            Some(time) => Ok(time),
            None => return Err(TransactionDetailError::TransactionDataError("Failed to get blocktime".to_string()))
        }
    }

    /// Return an array of processed token balances before the transaction was processed.
    /// Wrapped in `TransactionTokenProcessed` objects.
    pub fn get_token_balances_before(&self) -> Result<Vec<TransactionTokenProcessed>, TransactionDetailError> {
        let mut processed = vec![];
        let meta = self.get_meta().unwrap();
        match &meta.pre_token_balances {
            Some(token_data) => {
                for token in token_data {
                    let data = self.get_token_entry_processed(&token);
                    processed.push(data);
                }
                Ok(processed)
            },
            None => return Err(TransactionDetailError::TransactionMetaError("Failed to get pre token data".to_string()))
        }
    }

    /// Return an array of processed token balances after the transaction was processed.
    /// Wrapped in `TransactionTokenProcessed` objects.
    pub fn get_token_balances_after(&self) -> Result<Vec<TransactionTokenProcessed>, TransactionDetailError> {
        let mut processed = vec![];
        let meta = self.get_meta().unwrap();
        match &meta.post_token_balances {
            Some(token_data) => {
                for token in token_data {
                    let data = self.get_token_entry_processed(&token);
                    processed.push(data);
                }
                Ok(processed)
            },
            None => return Err(TransactionDetailError::TransactionMetaError("Failed to get post token data".to_string()))
        }
    }

    /// Return an array of instructions invoked during transaction processing.
    /// This includes the parent instructions (and their corresponding inner instructions)
    pub fn get_instructions_processed(&self) -> Vec<InstructionProcessed> {
        let mut instructions_processed = vec![];
        let account_keys = self.get_account_keys().unwrap();
        let block_time = self.get_block_time().unwrap();
        let transaction_signature = self.get_transaction_signature_id().unwrap();
        let instructions_parent = self.get_ixs().unwrap();
        let instructions_inner = self.get_ixs_inner().unwrap();

        // Looping over parent instructions!
        for (instruction_idx, instruction_parent) in instructions_parent.iter().enumerate() {

            let data = InstructionProcessed::new(&instruction_parent, &account_keys, &block_time, &transaction_signature);
            instructions_processed.push(data);

            // Looping over inner instructions if exist!
            //if let [_first, .., _intsruction_idx] = &instructions_inner[..] {
            if instructions_inner.len() > instruction_idx {
                for instruction_data in &instructions_inner[instruction_idx].instructions {
                    let data = InstructionProcessed::new(&instruction_data, &account_keys, block_time, &transaction_signature);
                    instructions_processed.push(data);
                }
            }
        }
        instructions_processed
    }

    /// Return transaction status metadata, `TransactionMetaData`, object.
    fn get_meta(&self) -> Result<&TransactionMetaData, TransactionDetailError> {
        match &self.meta {
            Some(meta) => Ok(meta),
            None => return Err(TransactionDetailError::TransactionMetaError("Failed to get meta data".to_string()))
        }
    }

    /// Return an array of `TransactionInstruction` objects.
    fn get_ixs(&self) -> Result<&Vec<TransactionInstructionData>, TransactionDetailError> {
        match &self.transaction {
            Some(ixs) => Ok(&ixs.message.instructions),
            None => return Err(TransactionDetailError::TransactionInstruction("Failed to get instructions".to_string()))
        }
    }

    /// Returns an array of `TransactionInnerInstruction` objects.
    fn get_ixs_inner(&self) -> Result<&Vec<TransactionInnerInstruction>, TransactionDetailError> {
        let meta = self.get_meta().unwrap();
        match &meta.inner_instructions {
            Some(ixs) => Ok(ixs),
            None => return Err(TransactionDetailError::TransactionInnerInstruction("Failed to get inner instructions".to_string()))
        }
    }

    /// Process `TransactionToken` into a public `TransactionTokenProcessed` wrapper.
    fn get_token_entry_processed(&self, entry: &TransactionToken) -> TransactionTokenProcessed {
        let accounts = self.get_account_keys().unwrap();
        TransactionTokenProcessed {
            token_account: accounts[entry.account_index].clone(),
            token_mint: entry.mint.clone(),
            owner: entry.owner.clone(),
            amount: entry.ui_token_amount.ui_amount_string.clone(),
            decimals: entry.ui_token_amount.decimals
        }
    }

    /// Process a raw transaction into a `TransactionDetail` object.
    fn process_transaction(signature: &str, client: &RpcClient) -> Result<Self, TransactionDetailError> {

        let signature = match Signature::from_str(signature) {
            Ok(signature) => signature,
            Err(err) => return Err(TransactionDetailError::ParseSignatureError(err.to_string()))
        };

        let tx = match client.get_transaction(&signature, UiTransactionEncoding::Json) {
            Ok(tx) => tx,
            Err(err) => return Err(TransactionDetailError::ClientError(err.to_string()))
        };

        let serialized = match serde_json::to_string(&tx) {
            Ok(serialized) => serialized,
            Err(err) => return Err(TransactionDetailError::SerializeError(err.to_string()))
        };
        
        let deserialized = match serde_json::from_str(&serialized) {
            Ok(deserialized) => deserialized,
            Err(err) => return Err(TransactionDetailError::DeserializeError(err.to_string()))  
        };

        Ok(deserialized)
    }
}

/// Transaction details for a confirmed transaction.
/// Contains the `slot`, `transaction`, `meta` and `block_time` fields.
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionDetail {
    /// The slot this transaction was processed in.
    slot: u64,
    /// Transaction object.
    transaction: Option<TransactionData>,
    /// Transaction status metadata object.
    meta: Option<TransactionMetaData>,
    #[serde(rename = "blockTime")]
    /// UNIX timestamp when transaction was processed.
    block_time: Option<u64>
}

/// Contained within an `TransactionDetail` object. Contains
/// `signatures` and `message` fields.
#[derive(Deserialize, Serialize, Debug)]
struct TransactionData {
    /// An array of base-58 encoded signatures applied to the transaction.
    signatures: Vec<String>,
    /// Content of the transaction.
    message: TransactionMessage
}

/// Contained within an `TransactionData` object.
/// Contains `header`, `account_keys`, `recent_blockhash` and `instructions` fields.
#[derive(Deserialize, Serialize, Debug)]
struct TransactionMessage {
    /// Details the account types and signatures required by the transaction.
    header: TransactionHeader,
    #[serde(rename = "accountKeys")]
    /// Array of base-58 encoded public keys used by the transaction, including by the instructions
    /// and for signatures.
    account_keys: Vec<String>,
    #[serde(rename = "recentBlockhash")]
    /// A base-58 encoded hash of a recent block in the ledger used to prevent transaction
    /// duplication and to give instructions lifetimes.
    recent_blockhash: String,
    /// Array of program instructions that will be executed in sequence and committed in one atomic
    /// transaction if all succeed.
    instructions: Vec<TransactionInstructionData>
}

/// Contained within an `TransactionMessage` object. Conaints `num_required_signatures`,
/// `num_readonly_signed_accounts` and `num_readonly_unsigned_accounts` fields.
#[derive(Deserialize, Serialize, Debug)]
struct TransactionHeader {
    #[serde(rename = "numRequiredSignatures")]
    /// The total number of signatures required to make the transaction valid. The signatures must match 
    /// the first `num_required_signatures` of `message.account_keys`.
    num_required_signatures: u32,
    #[serde(rename = "numReadonlySignedAccounts")]
    /// The last `num_readonly_signed_accounts` of the signed keys are read-only accounts. Programs may process
    /// multiple transactions that load read-only accounts within a single PoH entry, but are not permitted to 
    /// credit or debit lamports or modify account data. Transactions targeting the same read-write account are
    /// targeted sequentially. 
    num_readonly_signed_accounts: u32,
    #[serde(rename = "numReadonlyUnsignedAccounts")]
    /// The last `num_readonly_unsigned_accounts` of the unsigned keys are read-only accounts.
    num_readonly_unsigned_accounts: u32
}

/// Contained within an `TransactionDetail` object. Contains `fee`, `pre_balances`, `post_balances`,
/// `inner_instructions`, `log_messages`, `pre_token_balances` and `post_token_balances` fields.
#[derive(Deserialize, Serialize, Debug)]
struct TransactionMetaData {
    /// Fee this transaction was charged.
    fee: u64,
    #[serde(rename = "preBalances")]
    /// Array of u64 account balances from before the transaction was processed.
    pre_balances: Vec<u64>,
    #[serde(rename = "postBalances")]
    /// Array of u64 account balances after the transaction was processed.
    post_balances: Vec<u64>,
    #[serde(rename = "innerInstructions")]
    /// Array of inner instructions.
    inner_instructions: Option<Vec<TransactionInnerInstruction>>,
    #[serde(rename = "logMessages")]
    /// Array of string log messages.
    log_messages: Option<Vec<String>>,
    #[serde(rename = "preTokenBalances")]
    /// Array of token balances from before the transaction was processed.
    pre_token_balances: Option<Vec<TransactionToken>>,
    #[serde(rename = "postTokenBalances")]
    /// Array of token balances from after the transaction was processed.
    post_token_balances: Option<Vec<TransactionToken>>
}

/// Contained within an `TransactionMetaData` object. Contains `index` and `instructions` fields.
#[derive(Deserialize, Serialize, Debug)]
struct TransactionInnerInstruction {
    /// Index of the transaction instruction from which the inner instruction(s) originated.
    index: u32,
    /// Ordered array of inner program instructions that were invoked during a single transaction instruction.
    instructions: Vec<TransactionInstructionData>
}

/// Contained within an `TransactionMessage` and `TransactionInnerInstruction` object. Contains `program_id_index`, 
/// `accounts` and `data` fields.
#[derive(Deserialize, Serialize, Debug)]
struct TransactionInstructionData {
    #[serde(rename = "programIdIndex")]
    /// Index into the `message.account_keys` array indicating the program account that executes
    /// this instruction.
    program_id_index: u32,
    /// Array of ordered indices into the `message.account_keys` array indicating which accounts to pass
    /// to the program.
    accounts: Vec<u32>,
    /// The program input data encoded in a base-58 string.
    data: String
}

/// Contained within an `TransactionMetaData`. Contains `account_index`, `mint`, `ui_token_amount`
/// and `owner`.
#[derive(Deserialize, Serialize, Debug)]
struct TransactionToken {
    #[serde(rename = "accountIndex")]
    /// Index of the account in which the token balance is provided for.
    account_index: usize,
    /// Pubkey of the token's mint.
    mint: String,
    #[serde(rename = "uiTokenAmount")]
    /// Token mint balances of owner.
    ui_token_amount: TransactionTokenDetails,
    /// Pubkey of token balance's owner.
    owner: String
}

/// Contained within an `TransactionToken`. Contains `decimals`, `amount` and
/// `ui_amount_string` fields.
#[derive(Deserialize, Serialize, Debug)]
struct TransactionTokenDetails {
    /// Number of decimals configured for token's mint.
    decimals: Option<f32>,
    /// Raw amount of tokens as a string, ignoring decimals.
    amount: Option<String>,
    #[serde(rename = "uiAmountString")]
    /// Token amount as a string, accounting for decimals.
    ui_amount_string: Option<String>
}

/// Wrapper for `TransactionToken and TransactionTokenDetails` in a transaction.
/// Contains `token_account`, `token_mint`, `amount`, `decimals` and `owner` fields.
#[derive(Debug)]
pub struct TransactionTokenProcessed {
    /// Token account pubkey encoded as a base-58 string.
    pub token_account: String,
    /// Token mint pubkey encoded as a base-58 string.
    pub token_mint: String,
    /// Token account balance, accounting for decimals.
    pub amount: Option<String>,
    /// Token mint decimals.
    pub decimals: Option<f32>,
    /// Token account owner.
    pub owner: String,
}

/// Wrapper for `TransactionInstructionData`.
/// Contains `executer`, `accounts`, `data`, `block_time` and `signature` fields.
#[derive(Debug)]
pub struct InstructionProcessed {
    /// Program account that executed this instruction as a base-58 encoded string.
    pub executer: String,
    /// An array of accounts, as base-58 encoded strings, that were passed to the program.
    pub accounts: Vec<String>,
    /// Program input data encoded in a base-58 string.
    pub data: String,
    /// UNIX timestamp when transaction was processed.
    pub block_time: u64,
    /// Transaction signature id encoded as a base-58 string.
    pub signature: String
}

impl InstructionProcessed {
    /// Returns a new `InstructionProcessed` object.
    /// 
    /// # Arguments
    /// 
    /// * `instruction_data` - Instruction data within a transaction.
    /// * `account_keys` - An array of accounts participating within a transaction.
    /// * `block_time` - UNIX time when transaction was processed.
    /// * `signature` - Transaction signature id.
    /// ```
    fn new(instruction_data: &TransactionInstructionData, account_keys: &Vec<String>, block_time: &u64, signature: &String) -> Self {

        let mut accounts_participating = vec![];
        for account_idx in &instruction_data.accounts {
            let index = usize::try_from(*account_idx).unwrap();
            accounts_participating.push(account_keys[index].clone());
        }
        let exec_idx = usize::try_from(instruction_data.program_id_index).unwrap();
        let processed = InstructionProcessed {
            executer: account_keys[exec_idx].clone(),
            accounts: accounts_participating,
            data: instruction_data.data.clone(),
            block_time: block_time.clone(),
            signature: signature.clone()
        };
        processed
    }
}