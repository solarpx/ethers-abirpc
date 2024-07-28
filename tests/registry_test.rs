use ethers::prelude::abigen;
use ethers_abirpc::abirpc;

abigen!(Erc20Token, "./tests/abi/Erc20Token.abi");
abirpc!(Erc20Token, Erc20TokenRegistry);

use ethers::{
    contract::EthEvent,
    providers::{Http, MockProvider, RetryClient, Ws},
    types::BlockNumber,
};
use url::Url;

const TEST_ETHEREUM_WS_PROVIDER: &str = "wss://ethereum-rpc.publicnode.com";
const TEST_ETHEREUM_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";

const TEST_NETWORK: Network = Network::ETHEREUM;
const TEST_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"; // WETH

#[tokio::test]
async fn test_ws() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(TEST_ETHEREUM_WS_PROVIDER)?;
    let registry = Erc20TokenRegistry::<Ws>::new(Some(url), Some(TEST_NETWORK));
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(TEST_ETHEREUM_HTTP_PROVIDER)?;
    let registry = Erc20TokenRegistry::<Http>::new(Some(url), Some(TEST_NETWORK));
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_retry_client() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(TEST_ETHEREUM_HTTP_PROVIDER)?;
    let registry = Erc20TokenRegistry::<RetryClient<Http>>::new(Some(url), Some(TEST_NETWORK));
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_mock_provider() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<MockProvider>::new(None, None);
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().value(0_u64).tx;

    Ok(())
}

async fn get_logs<E>() -> Result<(), Box<dyn std::error::Error>>
where
    E: EthEvent + std::fmt::Debug,
{
    let url = Url::parse(TEST_ETHEREUM_WS_PROVIDER)?;
    let registry = Erc20TokenRegistry::<Ws>::new(Some(url.clone()), Some(TEST_NETWORK));
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance
        .get_logs::<E>(BlockNumber::Latest, BlockNumber::Latest)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_get_logs() -> Result<(), Box<dyn std::error::Error>> {
    get_logs::<TransferFilter>().await?;

    Ok(())
}
