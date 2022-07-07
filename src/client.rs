use core::time::Duration;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

/// RPC client configuration.
/// Contains `url`, `timeout` and `commitment` fields.
pub struct ClientConfig {
    pub url: String,
    pub timeout: Duration,
    pub commitment: CommitmentConfig
}

/// Returns RPC client.
/// 
/// # Arguments
/// 
/// * `config` - Client configuration.
pub fn get_client(config: ClientConfig) -> RpcClient {
    RpcClient::new_with_timeout_and_commitment(config.url, config.timeout, config.commitment)
}