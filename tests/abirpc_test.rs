use {ethers::prelude::abigen, ethers_abirpc::prelude::*};

abigen!(Erc20Token, "./tests/abi/Erc20Token.json");
abirpc!(Erc20Token, Erc20TokenRegistry);

abigen!(Erc721Token, "./tests/abi/Erc721Token.json");
abirpc!(Erc721Token, Erc721TokenRegistry);

#[tokio::test]
async fn test_abirpc() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
