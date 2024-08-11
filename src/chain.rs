use {
    crate::{error::Error, named::NamedChain},
    std::{clone::Clone, cmp},
    strum_macros::Display,
};

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
            initial_backoff_ms: 500,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub retry_client_config: RetryClientConfig,
    pub assert_chain_id: bool,
}

#[derive(Debug, PartialEq, Copy, Clone, Display)]
pub enum Chain {
    Id(u64),
    ChainConfig(ChainConfig),
}

impl From<NamedChain> for Chain {
    fn from(named: NamedChain) -> Self {
        Self::Id(named as u64)
    }
}

impl Chain {
    pub fn id(&self) -> u64 {
        match self {
            Chain::Id(chain_id) => *chain_id,
            Chain::ChainConfig(config) => config.chain_id,
        }
    }

    pub fn named(&self) -> Option<NamedChain> {
        match self {
            Chain::Id(chain_id) => NamedChain::try_from(*chain_id)
                .map_err(Error::NamedChainError)
                .ok(),
            Chain::ChainConfig(config) => NamedChain::try_from(config.chain_id)
                .map_err(Error::NamedChainError)
                .ok(),
        }
    }

    pub fn retry_client_config(&self) -> RetryClientConfig {
        match self {
            Chain::Id(chain_id) => match NamedChain::try_from(*chain_id) {
                Ok(named) => {
                    let default = RetryClientConfig::default().initial_backoff_ms;

                    let initial_backoff_ms = match named.average_blocktime_hint() {
                        Some(duration) => {
                            let interval = (duration.as_millis() as u64) / 10;
                            cmp::max(100, cmp::min(default, interval))
                        }
                        None => default,
                    };
                    RetryClientConfig {
                        initial_backoff_ms,
                        ..RetryClientConfig::default()
                    }
                }
                Err(_) => RetryClientConfig::default(),
            },
            Chain::ChainConfig(config) => config.retry_client_config,
        }
    }

    pub fn assert_chain_id(&self) -> bool {
        match self {
            Chain::Id(_) => true,
            Chain::ChainConfig(config) => config.assert_chain_id,
        }
    }
}
