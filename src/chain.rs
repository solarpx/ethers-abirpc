use {
    crate::{error::Error, named::NamedChain},
    ethers::types::U256,
    std::clone::Clone,
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
    pub chain_id: Option<u64>,
    pub retry_client_config: RetryClientConfig,
}

#[derive(Debug, PartialEq, Copy, Clone, Display)]
pub enum Chain {
    ChainId(u64),
    ChainConfig(ChainConfig),
}

impl From<NamedChain> for Chain {
    fn from(named: NamedChain) -> Self {
        Self::ChainId(named as u64)
    }
}

impl Chain {
    pub fn get_chainid(&self) -> Option<U256> {
        match self {
            Chain::ChainId(chain_id) => Some(U256::from(*chain_id)),
            Chain::ChainConfig(config) => config.chain_id.map(U256::from),
        }
    }

    pub fn as_named_chain(&self) -> Result<NamedChain, Error> {
        match self {
            Chain::ChainId(chain_id) => {
                NamedChain::try_from(*chain_id).map_err(|e| Error::NamedChainError(e))
            }
            Chain::ChainConfig(config) => match config.chain_id {
                Some(chain_id) => {
                    NamedChain::try_from(chain_id).map_err(|e| Error::NamedChainError(e))
                }
                None => Err(Error::Error(String::from("Configured chain_id is None"))),
            },
        }
    }

    pub fn retry_client_config(&self) -> RetryClientConfig {
        match self {
            Chain::ChainConfig(config) => config.retry_client_config,
            Chain::ChainId(chain_id) => match NamedChain::try_from(*chain_id) {
                Ok(named) => {
                    let initial_backoff_ms_default =
                        RetryClientConfig::default().initial_backoff_ms;

                    let initial_backoff_ms = match named.average_blocktime_hint() {
                        Some(duration) => {
                            initial_backoff_ms_default.min((duration.as_millis() as u64) / 2_u64)
                        }
                        None => initial_backoff_ms_default,
                    };
                    RetryClientConfig {
                        initial_backoff_ms,
                        ..RetryClientConfig::default()
                    }
                }
                Err(_) => RetryClientConfig::default(),
            },
        }
    }
}
