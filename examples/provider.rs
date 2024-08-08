use {
    ethers::providers::{Middleware, Provider, StreamExt, Ws},
    ethers_abirpc::prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider: Provider<Ws> = AbiProvider::new(
        Some(String::from("wss://ethereum-rpc.publicnode.com")),
        Some(Chain::ChainId(1)),
    )
    .provider()
    .await?;

    let mut stream = provider.subscribe_blocks().await?;
    if let Some(block) = stream.next().await {
        println!("{:?}", block)
    }

    Ok(())
}
