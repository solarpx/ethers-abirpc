use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./tests/abi/Erc20Token.json");
abirpc!(Erc20Token);

const TEST_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";
const TEST_WS_PROVIDER: &str = "wss://ethereum-rpc.publicnode.com";

const TEST_CHAIN: Chain = Chain::Id(1);
const TEST_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"; // WETH

#[tokio::test]
async fn test_ws() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<WsProvider>::new(TEST_WS_PROVIDER.into(), TEST_CHAIN);
    let provider = registry.provider().await?;
    let instance = registry.register(provider.clone(), address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<HttpProvider>::new(TEST_HTTP_PROVIDER.into(), TEST_CHAIN);
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_retry_client() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<RetryProvider>::new(TEST_HTTP_PROVIDER.into(), TEST_CHAIN);
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().await?;

    Ok(())
}

#[tokio::test]
async fn test_mock_provider() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<MockProvider>::mock();
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().value(0_u64).tx;

    Ok(())
}

async fn get_logs<E>() -> Result<(), Box<dyn std::error::Error>>
where
    E: EthEvent + std::fmt::Debug,
{
    let registry = Erc20TokenRegistry::<WsProvider>::new(TEST_WS_PROVIDER.into(), TEST_CHAIN);
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
