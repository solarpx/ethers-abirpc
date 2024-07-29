use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Abi error: {0}")]
    AbiError(#[from] ethers::abi::Error),
    #[error("Hex error: {0}")]
    AddressParseError(#[from] rustc_hex::FromHexError),
    #[error("ChainId error: {0}")]
    ChainIdError(String),
    #[error("Error: {0}")]
    Error(String),
    #[error("Provider error: {0}")]
    ProviderError(#[from] ethers::providers::ProviderError),
    #[error("Url parse Error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

impl<M> From<ethers::contract::ContractError<M>> for Error
where
    M: ethers::providers::Middleware,
{
    fn from(err: ethers::contract::ContractError<M>) -> Self {
        Self::Error(err.to_string())
    }
}
