use std::clone::Clone;
use strum_macros::Display;

#[derive(Debug, Clone)]
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
            initial_backoff_ms: 500,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Display)]
pub enum Network {
    ANVIL,
    ETHEREUM,
    OPTIMISM,
    ARBITRUM,
    ChainID(u32),
}

impl Network {
    pub fn chain_id(&self) -> u32 {
        match self {
            Network::ANVIL => 31337,
            Network::ETHEREUM => 1,
            Network::OPTIMISM => 10,
            Network::ARBITRUM => 42161,
            Network::ChainID(chain_id) => *chain_id,
        }
    }

    pub fn retry_client_config(&self) -> RetryClientConfig {
        match self {
            Network::ARBITRUM => RetryClientConfig {
                initial_backoff_ms: 150,
                ..RetryClientConfig::default()
            },
            _ => RetryClientConfig::default(),
        }
    }
}
