use ethers_abirpc::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider: WsProvider = AbiProvider::new(
        String::from("wss://polygon-bor-rpc.publicnode.com"),
        Chain::from(NamedChain::Polygon),
    )
    .provider()
    .await?;

    let mut stream = provider.subscribe_blocks().await?;
    if let Some(block) = stream.next().await {
        println!("{:?}", block)
    }

    Ok(())
}
