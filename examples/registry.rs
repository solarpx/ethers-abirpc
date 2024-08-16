use {
    ethers::{
        contract::abigen,
        providers::{Provider, Ws},
    },
    ethers_abirpc::prelude::*,
};

abigen!(Erc20Token, "./tests/abi/Erc20Token.json");
abirpc!(Erc20Token, Erc20TokenRegistry);

const WS_PROVIDER: &str = "wss://ethereum-rpc.publicnode.com";
const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const WBTC_ADDRESS: &str = "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<Provider<Ws>>::new(String::from(WS_PROVIDER), Chain::Id(1));

    let provider = registry.provider().await?;

    let weth_address = address_from!(WETH_ADDRESS)?;
    let wbtc_address = address_from!(WBTC_ADDRESS)?;

    let weth_instance = registry.register(provider.clone(), weth_address);
    let wbtc_instance = registry.register(provider.clone(), wbtc_address);

    let res = weth_instance.decimals().await?;
    println!("decimals (weth): {res:}");

    let res = wbtc_instance.decimals().await?;
    println!("decimals (wbtc): {res:}");

    Ok(())
}
