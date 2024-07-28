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
    pub url: Option<::url::Url>,
    pub network: Option<Network>,
    pub registry: Arc<RwLock<HashMap<Address, C>>>,
}

impl<C> AbiRegistry<C> {
    pub fn new(url: Option<Url>, network: Option<Network>) -> Self {
        Self {
            url,
            network,
            registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn entry_exists(&self, address: Address) -> bool {
        let arc_clone = Arc::clone(&self.registry);
        let registry = arc_clone.read().expect("Registry RwLock poisoned!");
        let entry_exists = registry.contains_key(&address);
        drop(registry);

        entry_exists
    }

    pub fn add_entry(&self, address: Address, contract: C) {
        let arc_clone = Arc::clone(&self.registry);
        let mut registry = arc_clone.write().expect("Registry RwLock poisoned!");
        registry.insert(address, contract);
        drop(registry);
    }
}

#[macro_export]
macro_rules! abirpc {
    ($abi:ident, $abi_registry: ident) => {
        use $crate::{address_from, network::Network, registry::AbiRegistryTrait};

        #[derive(Debug)]
        pub struct $abi_registry<T>(
            $crate::registry::AbiRegistry<$abi<::ethers::prelude::Provider<T>>>,
        )
        where
            T: ::ethers::prelude::JsonRpcClient;

        #[async_trait::async_trait]
        impl $crate::registry::AbiRegistryTrait<::ethers::prelude::Ws>
            for $abi_registry<::ethers::prelude::Ws>
        {
            async fn provider(
                &self,
            ) -> Result<::ethers::prelude::Provider<::ethers::prelude::Ws>, $crate::error::Error>
            {
                match &self.0.url {
                    Some(url) => {
                        let provider =
                            ::ethers::prelude::Provider::<::ethers::prelude::Ws>::connect(
                                url.clone(),
                            )
                            .await?;
                        Ok(provider)
                    }
                    None => Err($crate::error::Error::Error(String::from(
                        "Provider url is None",
                    ))),
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::registry::AbiRegistryTrait<::ethers::prelude::Http>
            for $abi_registry<::ethers::prelude::Http>
        {
            async fn provider(
                &self,
            ) -> Result<::ethers::prelude::Provider<::ethers::prelude::Http>, $crate::error::Error>
            {
                match &self.0.url {
                    Some(url) => {
                        let provider = ::ethers::prelude::Provider::<::ethers::prelude::Http>::new(
                            ::ethers::prelude::Http::new(url.clone()),
                        );
                        Ok(provider)
                    }
                    None => Err($crate::error::Error::Error(String::from(
                        "Provider url is None",
                    ))),
                }
            }
        }

        #[async_trait::async_trait]
        impl
            $crate::registry::AbiRegistryTrait<
                ::ethers::prelude::RetryClient<::ethers::prelude::Http>,
            > for $abi_registry<::ethers::prelude::RetryClient<::ethers::prelude::Http>>
        {
            async fn provider(
                &self,
            ) -> Result<
                ::ethers::prelude::Provider<
                    ::ethers::prelude::RetryClient<::ethers::prelude::Http>,
                >,
                $crate::error::Error,
            > {
                match &self.0.url {
                    Some(url) => {
                        let retry_config = match self.0.network {
                            Some(network) => network.retry_client_config(),
                            None => $crate::network::RetryClientConfig::default(),
                        };

                        let provider =
                            ::ethers::prelude::Provider::new(
                                ::ethers::prelude::RetryClientBuilder::default()
                                    .rate_limit_retries(retry_config.rate_limit_retries)
                                    .timeout_retries(retry_config.timeout_retries)
                                    .initial_backoff(::std::time::Duration::from_millis(
                                        retry_config.initial_backoff_ms,
                                    ))
                                    .build(
                                        ::ethers::prelude::Http::new(url.clone()),
                                        Box::new(
                                            ::ethers::prelude::HttpRateLimitRetryPolicy::default(),
                                        ),
                                    ),
                            );

                        Ok(provider)
                    }
                    None => Err($crate::error::Error::Error(String::from(
                        "Provider url is None",
                    ))),
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::registry::AbiRegistryTrait<::ethers::prelude::MockProvider>
            for $abi_registry<::ethers::prelude::MockProvider>
        {
            async fn provider(
                &self,
            ) -> Result<
                ::ethers::prelude::Provider<::ethers::prelude::MockProvider>,
                $crate::error::Error,
            > {
                match &self.0.url {
                    Some(_) => Err($crate::error::Error::Error(String::from(
                        "MockProvider url is not None",
                    ))),
                    None => {
                        let (provider, _mock) = ::ethers::prelude::Provider::mocked();
                        Ok(provider)
                    }
                }
            }
        }

        impl<T> $abi_registry<T>
        where
            T: ::ethers::prelude::JsonRpcClient,
        {
            pub fn new(url: Option<::url::Url>, network: Option<$crate::network::Network>) -> Self {
                let registry =
                    $crate::registry::AbiRegistry::<$abi<::ethers::prelude::Provider<T>>>::new(
                        url, network,
                    );
                Self(registry)
            }

            pub fn register(
                &self,
                provider: ::ethers::prelude::Provider<T>,
                address: ::ethers::prelude::Address,
            ) -> $abi<::ethers::prelude::Provider<T>> {
                if !self.0.entry_exists(address) {
                    let instance = $abi::new(address, provider.into());
                    self.0.add_entry(address, instance)
                }

                let clone_lock = std::sync::Arc::clone(&self.0.registry);
                let registry = clone_lock.read().expect("Registry RwLock poisoned!");
                let instance = registry.get(&address).unwrap().clone();
                drop(registry);

                instance
            }

            pub fn network(&self) -> Option<$crate::network::Network> {
                self.0.network
            }
        }

        impl<T> $abi<T>
        where
            T: ::ethers::prelude::Middleware,
        {
            pub async fn get_logs<E>(
                &self,
                from_block: ::ethers::prelude::BlockNumber,
                to_block: ::ethers::prelude::BlockNumber,
            ) -> Result<Vec<E>, $crate::error::Error>
            where
                E: ethers::prelude::EthEvent + std::fmt::Debug,
            {
                let res = self
                    .event::<E>()
                    .address(ethers::prelude::ValueOrArray::Value(self.address()))
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
        $address.parse::<ethers::prelude::Address>()
    };
}
