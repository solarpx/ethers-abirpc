use ethers_abirpc::prelude::*;

#[tokio::test]
async fn test_abigen() -> Result<(), Box<dyn std::error::Error>> {
    Abigen::new("ERC20Token", "./tests/abi/Erc20Token.json")?
        .generate()?
        .write_to_file("./tests/abi.rs")?;

    Ok(())
}
