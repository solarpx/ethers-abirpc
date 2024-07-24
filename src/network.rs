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
}

impl Network {
    pub fn chain_id(&self) -> u32 {
        match self {
            Network::ANVIL => 31337,
            Network::ETHEREUM => 1,
        }
    }

    pub fn retry_client_config(&self) -> RetryClientConfig {
        match self {
            Network::ANVIL => RetryClientConfig::default(),
            Network::ETHEREUM => RetryClientConfig::default(),
        }
    }
}
