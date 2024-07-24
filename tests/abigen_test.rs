use ethers::contract::Abigen;

#[tokio::test]
async fn test_abigen() -> Result<(), Box<dyn std::error::Error>> {
    Abigen::new("ERC20Token", "./src/abi/token/Erc20Token.abi")?
        .generate()?
        .write_to_file("./tests/abi.rs")?;

    Ok(())
}
