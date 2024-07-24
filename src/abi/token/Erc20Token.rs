use crate::{
    abirpc,
    error::Error,
    network::{Network, RetryClientConfig},
    registry::{AbiRegistry, AbiRegistryTrait},
};
use async_trait::async_trait;
use ethers::{
    prelude::abigen,
    providers::{
        Http, HttpRateLimitRetryPolicy, JsonRpcClient, Middleware, MockProvider, Provider,
        RetryClient, RetryClientBuilder, Ws,
    },
    types::{Address, BlockNumber, Bytes, TransactionReceipt},
};
use std::time::Duration;
use std::{clone::Clone, sync::Arc};
use url::Url;

#[derive(Debug)]
pub struct Erc20TokenRegistry<T>(AbiRegistry<Erc20Token<Provider<T>>>)
where
    T: JsonRpcClient;

abigen!(Erc20Token, "./src/abi/token/Erc20Token.abi");
abirpc!(Erc20Token, Erc20TokenRegistry);
