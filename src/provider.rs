use crate::{
    error::Error,
    network::{Network, RetryClientConfig},
};
use async_trait::async_trait;
use ethers::providers::{
    Http, HttpRateLimitRetryPolicy, Ipc, Middleware, MockProvider, Provider, RetryClient,
    RetryClientBuilder, Ws,
};
use std::{path::Path, time::Duration};
use url::Url;

#[async_trait]
pub trait AbiProviderTrait<M>
where
    M: Middleware,
{
    async fn provider(&self) -> Result<M, Error>;
}

pub struct AbiProvider {
    pub url: Option<String>,
    pub network: Option<Network>,
}

impl AbiProvider {
    pub fn new(url: Option<String>, network: Option<Network>) -> Self {
        Self { url, network }
    }
}

macro_rules! assert_chain_id {
    ($network: expr, $provider: expr) => {
        if let Some(network) = $network {
            let chain_id = $provider.get_chainid().await?;
            if network.get_chainid() != chain_id {
                let e = format!(
                    "Configured chain_id ({}) does not match network ({})",
                    network.get_chainid(),
                    chain_id
                );
                return Err(Error::ChainIdError(e));
            }
        }
    };
}

#[async_trait]
impl AbiProviderTrait<Provider<Ws>> for AbiProvider {
    async fn provider(&self) -> Result<Provider<Ws>, Error> {
        match &self.url {
            Some(url) => {
                let url = Url::parse(url)?;
                let provider = Provider::<Ws>::connect(url).await?;
                assert_chain_id!(self.network, provider);
                Ok(provider)
            }
            None => Err(Error::Error(String::from("Provider url is None"))),
        }
    }
}

#[async_trait]
impl AbiProviderTrait<Provider<Ipc>> for AbiProvider {
    async fn provider(&self) -> Result<Provider<Ipc>, Error> {
        match &self.url {
            Some(url) => {
                let provider = Provider::<Ipc>::connect_ipc(Path::new(&url)).await?;
                assert_chain_id!(self.network, provider);
                Ok(provider)
            }
            None => Err(Error::Error(String::from("Provider url is None"))),
        }
    }
}

#[async_trait]
impl AbiProviderTrait<Provider<Http>> for AbiProvider {
    async fn provider(&self) -> Result<Provider<Http>, Error> {
        match &self.url {
            Some(url) => {
                let url = Url::parse(url)?;
                let provider = Provider::<Http>::new(Http::new(url));
                assert_chain_id!(self.network, provider);
                Ok(provider)
            }
            None => Err(Error::Error(String::from("Provider url is None"))),
        }
    }
}

#[async_trait]
impl AbiProviderTrait<Provider<RetryClient<Http>>> for AbiProvider {
    async fn provider(&self) -> Result<Provider<RetryClient<Http>>, Error> {
        match &self.url {
            Some(url) => {
                let url = Url::parse(url)?;
                let retry_config = match self.network {
                    Some(network) => network.retry_client_config(),
                    None => RetryClientConfig::default(),
                };

                let provider = Provider::new(
                    RetryClientBuilder::default()
                        .rate_limit_retries(retry_config.rate_limit_retries)
                        .timeout_retries(retry_config.timeout_retries)
                        .initial_backoff(Duration::from_millis(retry_config.initial_backoff_ms))
                        .build(
                            Http::new(url),
                            Box::new(HttpRateLimitRetryPolicy::default()),
                        ),
                );
                assert_chain_id!(self.network, provider);
                Ok(provider)
            }
            None => Err(Error::Error(String::from("Provider url is None"))),
        }
    }
}

#[async_trait]
impl AbiProviderTrait<Provider<MockProvider>> for AbiProvider {
    async fn provider(&self) -> Result<Provider<MockProvider>, Error> {
        match &self.url {
            Some(_) => Err(Error::Error(String::from("MockProvider url is not None"))),
            None => {
                let (provider, _mock) = Provider::mocked();
                Ok(provider)
            }
        }
    }
}
