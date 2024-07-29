use ethers::providers::{Http, Middleware, MockProvider, Provider, RetryClient, Ws};
use ethers_abirpc::prelude::*;
use url::Url;

const TEST_ETHEREUM_WS_PROVIDER: &str = "wss://ethereum-rpc.publicnode.com";
const TEST_ETHEREUM_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";

const TEST_NETWORK: Network = Network::ChainId(1);

#[tokio::test]
async fn test_ws() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(TEST_ETHEREUM_WS_PROVIDER)?;
    let provider: Provider<Ws> = AbiProvider::new(Some(url), Some(TEST_NETWORK))
        .provider()
        .await?;

    let chain_id = provider.get_chainid().await?;

    assert_eq!(chain_id, TEST_NETWORK.get_chainid());

    Ok(())
}

#[tokio::test]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(TEST_ETHEREUM_HTTP_PROVIDER)?;
    let provider: Provider<Http> = AbiProvider::new(Some(url), Some(TEST_NETWORK))
        .provider()
        .await?;

    let chain_id = provider.get_chainid().await?;

    assert_eq!(chain_id, TEST_NETWORK.get_chainid());

    Ok(())
}

#[tokio::test]
async fn test_retry_client() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(TEST_ETHEREUM_HTTP_PROVIDER)?;
    let provider: Provider<RetryClient<Http>> = AbiProvider::new(Some(url), Some(TEST_NETWORK))
        .provider()
        .await?;

    let chain_id = provider.get_chainid().await?;

    assert_eq!(chain_id, TEST_NETWORK.get_chainid());

    Ok(())
}

#[tokio::test]
async fn test_mock_provider() -> Result<(), Box<dyn std::error::Error>> {
    let provider: Provider<MockProvider> = AbiProvider::new(None, None).provider().await?;

    let chain_id = provider.get_chainid().await;

    assert!(chain_id.is_err());

    Ok(())
}
