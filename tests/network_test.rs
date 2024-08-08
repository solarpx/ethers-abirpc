use {
    ethers::providers::{Http, Provider, RetryClient},
    ethers_abirpc::prelude::*,
};

const TEST_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";

#[tokio::test]
async fn test_chain_from_named_chain() -> Result<(), Box<dyn std::error::Error>> {
    let chain = Chain::from(NamedChain::Mainnet);

    let _provider: Provider<Http> = AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(chain))
        .provider()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_named_chain_from_chain() -> Result<(), Box<dyn std::error::Error>> {
    let chain = Chain::ChainId(1);

    let named = chain.as_named_chain()?;

    assert_eq!(named, NamedChain::Mainnet);

    Ok(())
}

#[tokio::test]
async fn test_chain_from_chain_id() -> Result<(), Box<dyn std::error::Error>> {
    let chain = Chain::ChainId(1);

    let _provider: Provider<Http> = AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(chain))
        .provider()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_chain_from_config() -> Result<(), Box<dyn std::error::Error>> {
    let chain = Chain::ChainConfig(ChainConfig {
        chain_id: Some(1),
        retry_client_config: RetryClientConfig {
            rate_limit_retries: 5,
            timeout_retries: 2,
            initial_backoff_ms: 200,
        },
    });

    let _provider: Provider<RetryClient<Http>> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(chain))
            .provider()
            .await?;

    Ok(())
}

#[tokio::test]
async fn test_chain_from_config_default() -> Result<(), Box<dyn std::error::Error>> {
    let chain = Chain::ChainConfig(ChainConfig::default());

    let _provider: Provider<RetryClient<Http>> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(chain))
            .provider()
            .await?;

    assert_eq!(chain.get_chainid(), None);

    Ok(())
}
