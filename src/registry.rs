use crate::{error::Error, network::Network};
use async_trait::async_trait;
use ethers::{
    providers::{JsonRpcClient, Provider},
    types::Address,
};
use std::{
    clone::Clone,
    collections::HashMap,
    sync::{Arc, RwLock},
};
use url::Url;

#[async_trait]
pub trait AbiRegistryTrait<T>
where
    T: JsonRpcClient,
{
    async fn provider(&self) -> Result<Provider<T>, Error>;
}

#[derive(Debug)]
pub struct AbiRegistry<C> {
    pub url: Option<Url>,
    pub network: Option<Network>,
    pub(crate) registry: Arc<RwLock<HashMap<Address, C>>>,
}

impl<C> AbiRegistry<C> {
    pub fn new(url: Option<Url>, network: Option<Network>) -> Self {
        Self {
            url,
            network,
            registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) fn entry_exists(&self, address: Address) -> bool {
        let arc_clone = Arc::clone(&self.registry);
        let registry = arc_clone.read().expect("Registry RwLock poisoned!");
        let entry_exists = registry.contains_key(&address);
        drop(registry);

        entry_exists
    }

    pub(crate) fn add_entry(&self, address: Address, contract: C) {
        let arc_clone = Arc::clone(&self.registry);
        let mut registry = arc_clone.write().expect("Registry RwLock poisoned!");
        registry.insert(address, contract);
        drop(registry);
    }
}

#[macro_export]
macro_rules! abirpc {
    ($abi:ident, $abi_registry: ident) => {
        #[async_trait]
        impl AbiRegistryTrait<Ws> for $abi_registry<Ws> {
            async fn provider(&self) -> Result<Provider<Ws>, Error> {
                match &self.0.url {
                    Some(url) => {
                        let provider = Provider::<Ws>::connect(url.clone()).await?;
                        Ok(provider)
                    }
                    None => Err(Error::Error(String::from("Provider url is None"))),
                }
            }
        }

        #[async_trait]
        impl AbiRegistryTrait<Http> for $abi_registry<Http> {
            async fn provider(&self) -> Result<Provider<Http>, Error> {
                match &self.0.url {
                    Some(url) => {
                        let provider = Provider::<Http>::new(Http::new(url.clone()));
                        Ok(provider)
                    }
                    None => Err(Error::Error(String::from("Provider url is None"))),
                }
            }
        }

        #[async_trait]
        impl AbiRegistryTrait<RetryClient<Http>> for $abi_registry<RetryClient<Http>> {
            async fn provider(&self) -> Result<Provider<RetryClient<Http>>, Error> {
                match &self.0.url {
                    Some(url) => {
                        let retry_config = match self.0.network {
                            Some(network) => network.retry_client_config(),
                            None => RetryClientConfig::default(),
                        };

                        let provider = Provider::new(
                            RetryClientBuilder::default()
                                .rate_limit_retries(retry_config.rate_limit_retries)
                                .timeout_retries(retry_config.timeout_retries)
                                .initial_backoff(Duration::from_millis(
                                    retry_config.initial_backoff_ms,
                                ))
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
        impl AbiRegistryTrait<MockProvider> for $abi_registry<MockProvider> {
            async fn provider(&self) -> Result<Provider<MockProvider>, Error> {
                match &self.0.url {
                    Some(_) => Err(Error::Error(String::from("MockProvider url is not None"))),
                    None => {
                        let (provider, _mock) = Provider::mocked();
                        Ok(provider)
                    }
                }
            }
        }

        impl<T> $abi_registry<T>
        where
            T: JsonRpcClient,
        {
            pub fn new(url: Option<Url>, network: Option<Network>) -> Self {
                let registry = AbiRegistry::<$abi<Provider<T>>>::new(url, network);
                Self(registry)
            }

            pub fn register(&self, provider: Provider<T>, address: Address) -> $abi<Provider<T>> {
                if !self.0.entry_exists(address) {
                    let instance = $abi::new(address, provider.into());
                    self.0.add_entry(address, instance)
                }

                let clone_lock = Arc::clone(&self.0.registry);
                let registry = clone_lock.read().expect("Registry RwLock poisoned!");
                let instance = registry.get(&address).unwrap().clone();
                drop(registry);

                instance
            }

            pub fn network(&self) -> Option<Network> {
                self.0.network
            }

            pub async fn send_raw_transaction(
                &self,
                provider: Provider<T>,
                tx: Bytes,
                await_receipt: bool,
            ) -> Result<Option<TransactionReceipt>, Error> {
                let pending_tx = provider.send_raw_transaction(tx).await?;
                if await_receipt {
                    let receipt = pending_tx.await?;
                    return Ok(receipt);
                }

                Ok(None)
            }
        }

        impl<T> $abi<T>
        where
            T: Middleware,
        {
            pub async fn get_logs<E>(
                &self,
                from_block: BlockNumber,
                to_block: BlockNumber,
            ) -> Result<Vec<E>, Error>
            where
                E: ethers::prelude::EthEvent + std::fmt::Debug,
            {
                let res = self
                    .event::<E>()
                    .address(ethers::types::ValueOrArray::Value(self.address()))
                    .from_block(from_block)
                    .to_block(to_block)
                    .query()
                    .await?;

                Ok(res)
            }
        }
    };
}

#[macro_export]
macro_rules! address_from {
    ($address: expr) => {
        $address.parse::<Address>()
    };
}
