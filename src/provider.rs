use crate::{
    error::Error,
    network::{Network, RetryClientConfig},
};
use async_trait::async_trait;
use ethers::providers::{
    Http, HttpRateLimitRetryPolicy, JsonRpcClient, MockProvider, Provider, RetryClient,
    RetryClientBuilder, Ws,
};
use std::time::Duration;
use url::Url;

#[async_trait]
pub trait AbiProviderTrait<T>
where
    T: JsonRpcClient,
{
    async fn provider(&self) -> Result<Provider<T>, Error>;
}

pub struct AbiProvider {
    pub url: Option<Url>,
    pub network: Option<Network>,
}

impl AbiProvider {
    pub fn new(url: Option<Url>, network: Option<Network>) -> Self {
        Self { url, network }
    }
}

#[async_trait]
impl AbiProviderTrait<Ws> for AbiProvider {
    async fn provider(&self) -> Result<Provider<Ws>, Error> {
        match &self.url {
            Some(url) => {
                let provider = Provider::<Ws>::connect(url.clone()).await?;
                Ok(provider)
            }
            None => Err(Error::Error(String::from("Provider url is None"))),
        }
    }
}

#[async_trait]
impl AbiProviderTrait<Http> for AbiProvider {
    async fn provider(&self) -> Result<Provider<Http>, Error> {
        match &self.url {
            Some(url) => {
                let provider = Provider::<Http>::new(Http::new(url.clone()));
                Ok(provider)
            }
            None => Err(Error::Error(String::from("Provider url is None"))),
        }
    }
}

#[async_trait]
impl AbiProviderTrait<RetryClient<Http>> for AbiProvider {
    async fn provider(&self) -> Result<Provider<RetryClient<Http>>, Error> {
        match &self.url {
            Some(url) => {
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
                            Http::new(url.clone()),
                            Box::new(HttpRateLimitRetryPolicy::default()),
                        ),
                );

                Ok(provider)
            }
            None => Err(Error::Error(String::from("Provider url is None"))),
        }
    }
}

#[async_trait]
impl AbiProviderTrait<MockProvider> for AbiProvider {
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
