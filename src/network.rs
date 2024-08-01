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

#[derive(Debug, PartialEq, Copy, Clone, Display)]
pub enum Network {
    ANVIL,
    ARBITRUM,
    AVALANCHE,
    BASE,
    BLAST,
    BSC,
    CELO,
    ETHEREUM,
    FANTOM,
    FILECOIN,
    GNOSIS,
    IOTEX,
    KAVA,
    KLAYTN,
    LINEA,
    METIS,
    MOONBEAM,
    MOONRIVER,
    OPTIMISM,
    POLYGON,
    Anonymous,
    ChainId(u32),
    NetworkConfig(NetworkConfig),
}

impl Network {
    pub fn get_chainid(&self) -> Option<U256> {
        match self {
            Network::ANVIL => Some(U256::from(31337)),
            Network::ARBITRUM => Some(U256::from(42161)),
            Network::AVALANCHE => Some(U256::from(43114)),
            Network::BASE => Some(U256::from(8453)),
            Network::BLAST => Some(U256::from(81457)),
            Network::BSC => Some(U256::from(56)),
            Network::CELO => Some(U256::from(42220)),
            Network::ETHEREUM => Some(U256::from(1)),
            Network::FANTOM => Some(U256::from(250)),
            Network::FILECOIN => Some(U256::from(314)),
            Network::GNOSIS => Some(U256::from(100)),
            Network::IOTEX => Some(U256::from(4689)),
            Network::KAVA => Some(U256::from(2222)),
            Network::KLAYTN => Some(U256::from(8217)),
            Network::LINEA => Some(U256::from(59144)),
            Network::METIS => Some(U256::from(1088)),
            Network::MOONBEAM => Some(U256::from(1284)),
            Network::MOONRIVER => Some(U256::from(1285)),
            Network::OPTIMISM => Some(U256::from(10)),
            Network::POLYGON => Some(U256::from(137)),
            Network::Anonymous => None,
            Network::ChainId(chain_id) => Some(U256::from(*chain_id)),
            Network::NetworkConfig(config) => config.chain_id.map(|chain_id| U256::from(chain_id)),
        }
    }

    pub fn retry_client_config(&self) -> RetryClientConfig {
        match self {
            Network::ARBITRUM => RetryClientConfig {
                initial_backoff_ms: 150,
                ..RetryClientConfig::default()
            },
            Network::NetworkConfig(config) => config.retry_client_config,
            _ => RetryClientConfig::default(),
        }
    }
}
