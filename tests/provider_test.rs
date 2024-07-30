use ethers::providers::{Http, MockProvider, Provider, RetryClient, Ws};
use ethers_abirpc::prelude::*;

const TEST_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";
const TEST_WS_PROVIDER: &str = "wss://ethereum-rpc.publicnode.com";
const TEST_NETWORK: Network = Network::ChainId(1);

#[tokio::test]
async fn test_ws() -> Result<(), Box<dyn std::error::Error>> {
    let _provider: Provider<Ws> =
        AbiProvider::new(Some(TEST_WS_PROVIDER.into()), Some(TEST_NETWORK))
            .provider()
            .await?;

    Ok(())
}

#[tokio::test]
async fn test_ws_wrong_url() -> Result<(), Box<dyn std::error::Error>> {
    let provider: Result<Provider<Ws>, _> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(TEST_NETWORK))
            .provider()
            .await;

    assert!(provider.is_err());

    Ok(())
}

#[tokio::test]
async fn test_ws_wrong_chain_id() -> Result<(), Box<dyn std::error::Error>> {
    let provider: Result<Provider<Ws>, _> =
        AbiProvider::new(Some(TEST_WS_PROVIDER.into()), Some(Network::ChainId(10)))
            .provider()
            .await;

    assert!(provider.is_err());

    Ok(())
}

#[tokio::test]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let _provider: Provider<Http> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(TEST_NETWORK))
            .provider()
            .await?;

    Ok(())
}

#[tokio::test]
async fn test_retry_client() -> Result<(), Box<dyn std::error::Error>> {
    let _provider: Provider<RetryClient<Http>> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(TEST_NETWORK))
            .provider()
            .await?;

    Ok(())
}

#[tokio::test]
async fn test_mock_provider() -> Result<(), Box<dyn std::error::Error>> {
    let _provider: Provider<MockProvider> = AbiProvider::new(None, None).provider().await?;

    Ok(())
}
