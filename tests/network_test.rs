use ethers::providers::{Http, Provider, RetryClient};
use ethers_abirpc::prelude::*;

const TEST_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";

#[tokio::test]
async fn test_network_from_keyword() -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::ETHEREUM;

    let _provider: Provider<Http> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(network))
            .provider()
            .await?;

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
async fn test_network_from_chain_id_with_retry_config() -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::ChainIdWithRetryClientConfig((
        1,
        RetryClientConfig {
            rate_limit_retries: 5,
            timeout_retries: 2,
            initial_backoff_ms: 200,
        },
    ));

    let _provider: Provider<RetryClient<Http>> =
        AbiProvider::new(Some(TEST_HTTP_PROVIDER.into()), Some(network))
            .provider()
            .await?;

    Ok(())
}
