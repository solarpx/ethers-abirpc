use ethers::types::U256;
use std::clone::Clone;
use strum_macros::Display;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RetryClientConfig {
    pub rate_limit_retries: u32,
    pub timeout_retries: u32,
    pub initial_backoff_ms: u64,
}

impl Default for RetryClientConfig {
    fn default() -> Self {
        Self {
            rate_limit_retries: 10,
            timeout_retries: 3,
            initial_backoff_ms: 300,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct NetworkConfig {
    pub chain_id: Option<u32>,
    pub retry_client_config: RetryClientConfig,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            chain_id: None,
            retry_client_config: RetryClientConfig::default(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Display)]
pub enum Network {
    ChainId(u32),
    NetworkConfig(NetworkConfig),
}

impl Network {
    pub fn get_chainid(&self) -> Option<U256> {
        match self {
            Network::ChainId(chain_id) => Some(U256::from(*chain_id)),
            Network::NetworkConfig(config) => config.chain_id.map(U256::from),
        }
    }

    pub fn retry_client_config(&self) -> RetryClientConfig {
        match self {
            Network::NetworkConfig(config) => config.retry_client_config,
            _ => RetryClientConfig::default(),
        }
    }
}
