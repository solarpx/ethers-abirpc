use ethers::types::U256;
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
            initial_backoff_ms: 300,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Display)]
pub enum Network {
    ANVIL,
    ETHEREUM,
    OPTIMISM,
    BSC,
    GNOSIS,
    POLYGON,
    FANTOM,
    FILECOIN,
    METIS,
    MOONBEAM,
    MOONRIVER,
    KAVA,
    IOTEX,
    KLAYTN,
    BASE,
    ARBITRUM,
    CELO,
    AVALANCHE,
    LINEA,
    BLAST,
    ChainId(u32),
}

impl Network {
    pub fn get_chainid(&self) -> U256 {
        match self {
            Network::ETHEREUM => U256::from(1),
            Network::OPTIMISM => U256::from(10),
            Network::BSC => U256::from(56),
            Network::GNOSIS => U256::from(100),
            Network::POLYGON => U256::from(137),
            Network::FANTOM => U256::from(250),
            Network::FILECOIN => U256::from(314),
            Network::METIS => U256::from(1088),
            Network::MOONBEAM => U256::from(1284),
            Network::MOONRIVER => U256::from(1285),
            Network::KAVA => U256::from(2222),
            Network::IOTEX => U256::from(4689),
            Network::KLAYTN => U256::from(8217),
            Network::BASE => U256::from(8453),
            Network::ANVIL => U256::from(31337),
            Network::ARBITRUM => U256::from(42161),
            Network::CELO => U256::from(42220),
            Network::AVALANCHE => U256::from(43114),
            Network::LINEA => U256::from(59144),
            Network::BLAST => U256::from(81457),
            Network::ChainId(chain_id) => U256::from(*chain_id),
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
