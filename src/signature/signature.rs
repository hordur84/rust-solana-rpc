use std::str::FromStr;
use core::fmt;
use solana_client::rpc_client::{RpcClient, GetConfirmedSignaturesForAddress2Config};
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use crate::parsing::time::{convert_time_to_unix, convert_unix_to_time};

/// Wrapper for transaction signature information, `RpcConfirmedTransactionStatusWithSignature`.
/// Contains `signature`, `slot`, `block_time` and `block_time_human` fields.
pub struct SignatureDetail {
    /// Transaction signature as a base-58 encoded string.
    pub signature: String,
    /// The slot that contains the block with the transaction.
    pub slot: u64,
    /// Estimated production time, as a UNIX timestamp, of when transaction
    /// was processed.
    pub block_time: Option<i64>,
    /// Time formatted string, taken from `block_time`.
    pub block_time_human: Option<String>
}

impl fmt::Display for SignatureDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "signature: {} - time: {:?}", self.signature, self.block_time_human)
    }
}

/// Wrapper for `GetConfirmedSignaturesForAddress2Config`.
/// Contains `time_before`, `time_after`, `before`, `until`, `limit` and `commitment` fields.
pub struct SignatureConfig {
    /// Same as `before`, except instead of a `Signature`, uses an explicity formatted time string, i.e.
    /// `Y-M-D H:M:S`.
    pub time_before: Option<String>,
    /// Same as `after`, except instead of a `Signature`, uses an explicity formatted time string, i.e.
    /// `Y-M-D H:M:S`.
    pub time_after: Option<String>,
    /// Start searching backwards from this signature. If not provided the search 
    /// starts from the top of the highest max confirmed block.
    pub before: Option<Signature>,
    /// Search until this transaction signature, if found before limit reached.
    pub until: Option<Signature>,
    /// Maximum transaction signatures to return (between 1 - 1000).
    pub limit: Option<usize>, 
    /// Commitment describes how finalized a block is at a point in time, can be
    /// `finalized`, `confirmed` or `processed`.
    pub commitment: Option<CommitmentConfig>,
}

/// Fetching and processing of transaction signatures from an address.
pub struct Signatures {}

impl Signatures {
    /// Return transaction signatures for a given account based on the specified configuration.
    /// # Arguments
    /// 
    /// * `account` - The account to fetch the transaction signatures for.
    /// * `client` - RPC client.
    /// * `number` - Amount of transaction signatures to fetch.
    pub fn fetch(account: &String, client: &RpcClient, config: &SignatureConfig) -> Option<Vec<SignatureDetail>> {

        let pubkey = Pubkey::from_str(&account).expect("Failed to parse account address");
        let conf = GetConfirmedSignaturesForAddress2Config {
            before: config.before,
            until: config.until,
            limit: config.limit,
            commitment: config.commitment
        };

        let signatures = client.get_signatures_for_address_with_config(&pubkey, conf).unwrap();
        let signatures = Self::get_processed_signatures(signatures);

        let time_before = config.time_before.clone();
        let time_after = config.time_after.clone();
        let filter_by_time = time_before.or(time_after).is_some();

        if filter_by_time {
            Self::filter(signatures, config)
        }
        else {
            Some(signatures)
        }
    }

    /// Filter the specified signatures according to the specified configuration.
    fn filter(signatures: Vec<SignatureDetail>, config: &SignatureConfig) -> Option<Vec<SignatureDetail>> {

        let signatures = signatures.into_iter().filter(|x| {
            let block_time = u64::try_from(x.block_time.unwrap()).unwrap();
            if let Some(time_start) = &config.time_before {
                let time_start = convert_time_to_unix(time_start.to_string());
                if let Some(time_end) = &config.time_after {
                    let time_end = convert_time_to_unix(time_end.to_string());
                    if (block_time < time_end) && (block_time > time_start) {
                        return true;
                    }
                    else {
                        return false;
                    }
                }
                else {
                    if block_time > time_start {
                        return true;
                    }
                    else {
                        return false;
                    }
                }
            }
            else {
                if let Some(time_end) = &config.time_after {
                    let time_end = convert_time_to_unix(time_end.to_string());
                    if block_time < time_end {
                        return true;
                    }
                    else {
                        return false;
                    }
                }
                else {
                    return false;
                }
            }
        });
        let signatures: Vec<SignatureDetail> = signatures.collect();
        Some(signatures)
    }

    /// Returns an array of processed transaction signatures, as `SignatureDetail` objects.
    fn get_processed_signatures(signatures_raw: Vec<RpcConfirmedTransactionStatusWithSignature>) -> Vec<SignatureDetail> {
        let mut processed = vec![];
        for signature in signatures_raw {
            let data = SignatureDetail {
                signature: signature.signature,
                slot: signature.slot,
                block_time: signature.block_time,
                block_time_human: Some(convert_unix_to_time(u64::try_from(signature.block_time.unwrap()).unwrap()))
            };
            processed.push(data);
        }
        processed
    }
}