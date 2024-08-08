use {
    ethers::{
        contract::{abigen, EthEvent},
        providers::{Http, MockProvider, Provider, RetryClient, Ws},
        types::BlockNumber,
    },
    ethers_abirpc::prelude::*,
};

abigen!(Erc20Token, "./tests/abi/Erc20Token.abi");
abirpc!(Erc20Token, Erc20TokenRegistry);

const TEST_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";
const TEST_WS_PROVIDER: &str = "wss://ethereum-rpc.publicnode.com";

const TEST_CHAIN: Chain = Chain::ChainId(1);
const TEST_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"; // WETH

#[tokio::test]
async fn test_ws() -> Result<(), Box<dyn std::error::Error>> {
    let registry =
        Erc20TokenRegistry::<Provider<Ws>>::new(Some(TEST_WS_PROVIDER.into()), Some(TEST_CHAIN));
    let provider = registry.provider().await?;
    let instance = registry.register(provider.clone(), address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<Provider<Http>>::new(
        Some(TEST_HTTP_PROVIDER.into()),
        Some(TEST_CHAIN),
    );
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_retry_client() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<Provider<RetryClient<Http>>>::new(
        Some(TEST_HTTP_PROVIDER.into()),
        Some(TEST_CHAIN),
    );
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_mock_provider() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<Provider<MockProvider>>::new(None, None);
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().value(0_u64).tx;

    Ok(())
}

async fn get_logs<E>() -> Result<(), Box<dyn std::error::Error>>
where
    E: EthEvent + std::fmt::Debug,
{
    let registry =
        Erc20TokenRegistry::<Provider<Ws>>::new(Some(TEST_WS_PROVIDER.into()), Some(TEST_CHAIN));
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
