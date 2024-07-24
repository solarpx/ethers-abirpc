use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Error(String),
    #[error("Contract error: {0}")]
    ContractError(String),
    #[error("Abi error: {0}")]
    AbiError(#[from] ethers::abi::Error),
    #[error("Hex error: {0}")]
    AddressParseError(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    ProviderError(#[from] ethers::providers::ProviderError),
    #[error("Url parse Error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

impl<M> From<ethers::contract::ContractError<M>> for Error
where
    M: ethers::middleware::Middleware,
{
    fn from(err: ethers::contract::ContractError<M>) -> Self {
        Self::ContractError(err.to_string())
    }
}
