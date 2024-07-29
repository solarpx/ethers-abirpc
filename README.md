# ethers-abirpc

This library defines the `abirpc!` macro along with several other utilites for ethers-rs provider encapsulation. The following ethers-rs provider types are supported

```rust
Provider<Ws>
Provider<Http>
Provider<RetryClient<Http>>
Provider<MockProvider>
```

The `abirpc!` macro is implemented as an extension of ethers-rs `abigen!`. `abirpc!` generates an API to manage deployed contract instances, and `abigen!` provides the rust bindings for the contract ABI. 

```rust
use ethers::{
    contract::abigen,
    providers::{Provider, Ws},
};
use ethers_abirpc::prelude::*;
use url::Url;

abigen!(Erc20Token, "./tests/abi/Erc20Token.abi"); // Path to abi (json)
abirpc!(Erc20Token, Erc20TokenRegistry);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = address_from!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?; // WETH
    let url = Url::parse("wss://ethereum-rpc.publicnode.com")?;

    let registry = Erc20TokenRegistry::<Provider<Ws>>::new(Some(url), Some(Network::ETHEREUM));
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address);

    let _ = instance.decimals().await?; // Query contract abi

    Ok(())
}
```

The crate also includes wrappers for direct initialization of supported ethers-rs providers. This can be helpful for interacting with ethers-rs primitives and middleware. 

```rust
let url = Url::parse("wss://ethereum-rpc.publicnode.com")?;
let provider: Provider<Ws> = AbiProvider::new(Some(url), Some(Network::ChainId(1)))
    .provider()
    .await?;

let _ = provider.get_chainid().await?;
```
