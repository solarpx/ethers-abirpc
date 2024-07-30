use ethers::prelude::abigen;
use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./tests/abi/Erc20Token.abi");
abirpc!(Erc20Token, Erc20TokenRegistry);

abigen!(Erc721Token, "./tests/abi/Erc721Token.abi");
abirpc!(Erc721Token, Erc721TokenRegistry);

#[tokio::test]
async fn test_abirpc() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
