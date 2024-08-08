use {
    ethers::providers::{Http, Provider, RetryClient},
    ethers_abirpc::prelude::*,
};

const TEST_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";

#[tokio::test]
async fn test_network_from_named_chain() -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::from(NamedChain::Mainnet);

    let _provider: Provider<Http> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(network))
            .provider()
            .await?;

    Ok(())
}

#[tokio::test]
async fn test_named_chain_from_network() -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::ChainId(1);

    let named = network.as_named_chain()?;

    assert_eq!(named, NamedChain::Mainnet);

    Ok(())
}

#[tokio::test]
async fn test_network_from_chain_id() -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::ChainId(1);

    let _provider: Provider<Http> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(network))
            .provider()
            .await?;

    Ok(())
}

#[tokio::test]
async fn test_network_from_config() -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::NetworkConfig(NetworkConfig {
        chain_id: Some(1),
        retry_client_config: RetryClientConfig {
            rate_limit_retries: 5,
            timeout_retries: 2,
            initial_backoff_ms: 200,
        },
    });

    let _provider: Provider<RetryClient<Http>> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(network))
            .provider()
            .await?;

    Ok(())
}

#[tokio::test]
async fn test_network_from_config_default() -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::NetworkConfig(NetworkConfig::default());

    let _provider: Provider<RetryClient<Http>> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(network))
            .provider()
            .await?;

    assert_eq!(network.get_chainid(), None);

    Ok(())
}
