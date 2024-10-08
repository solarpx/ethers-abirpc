use ethers_abirpc::prelude::*;

const TEST_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";
const TEST_WS_PROVIDER: &str = "wss://ethereum-rpc.publicnode.com";
const TEST_CHAIN: Chain = Chain::Id(1);

#[tokio::test]
async fn test_ws() -> Result<(), Box<dyn std::error::Error>> {
    let _provider: WsProvider = AbiProvider::new(TEST_WS_PROVIDER.into(), TEST_CHAIN)
        .provider()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_ws_wrong_url() -> Result<(), Box<dyn std::error::Error>> {
    let provider: Result<WsProvider, _> = AbiProvider::new(TEST_HTTP_PROVIDER.into(), TEST_CHAIN)
        .provider()
        .await;

    assert!(provider.is_err());

    Ok(())
}

#[tokio::test]
async fn test_ws_wrong_chain_id() -> Result<(), Box<dyn std::error::Error>> {
    let provider: Result<WsProvider, _> =
        AbiProvider::new(TEST_WS_PROVIDER.into(), Chain::from_id(10))
            .provider()
            .await;

    assert!(provider.is_err());

    Ok(())
}

#[tokio::test]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let _provider: HttpProvider = AbiProvider::new(TEST_HTTP_PROVIDER.into(), TEST_CHAIN)
        .provider()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_retry_client() -> Result<(), Box<dyn std::error::Error>> {
    let _provider: RetryProvider = AbiProvider::new(TEST_HTTP_PROVIDER.into(), TEST_CHAIN)
        .provider()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_mock_provider() -> Result<(), Box<dyn std::error::Error>> {
    let _provider: MockProvider = AbiProvider::mock().provider().await?;

    Ok(())
}
