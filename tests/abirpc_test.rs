use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./tests/abi/Erc20Token.json");
abirpc!(Erc20Token);

abigen!(Erc721Token, "./tests/abi/Erc721Token.json");
abirpc!(Erc721Token);

#[tokio::test]
async fn test_abirpc() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
