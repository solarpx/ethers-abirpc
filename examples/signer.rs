use {
    ethers::{
        contract::abigen,
        middleware::SignerMiddleware,
        providers::{Middleware, MockProvider, Provider},
        signers::{LocalWallet, Signer},
        types::U256,
    },
    ethers_abirpc::prelude::*,
};

abigen!(Erc20Token, "./tests/abi/Erc20Token.json");
abirpc!(Erc20Token, Erc20TokenRegistry);

const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const LOCAL_WALLET: &str = "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Erc20TokenRegistry::<Provider<MockProvider>>::new(None, None);

    let weth_address = address_from!(WETH_ADDRESS)?;

    let provider = registry.provider().await?;
    let instance = registry.register(provider.clone(), weth_address);

    let wallet: LocalWallet = LOCAL_WALLET.parse()?;
    let weth_amount = U256::from(10u64.pow(15));
    let tx = instance
        .approve(wallet.address(), weth_amount)
        .value(0_u64)
        .tx;

    let signer = SignerMiddleware::new(provider, wallet);
    let signed_tx = signer.sign_transaction(&tx, weth_address).await?;

    println!("{:?}", tx);
    println!("{:?}", signed_tx);

    Ok(())
}
