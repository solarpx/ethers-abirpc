# ethers-abirpc

This crate defines the `abirpc!` macro along with several other utilites for [ethers-rs](https://github.com/gakonst/ethers-rs) provider encapsulation. The `abirpc!` macro is implemented as an extension of ethers-rs `abigen!`. The following ethers-rs provider types are supported:

```rust
Provider<Ws>
Provider<Http>
Provider<RetryClient<Http>>
Provider<Ipc>
Provider<MockProvider>
```
### Usage 

`abirpc!` generates an API to manage deployed contract instances as shown in the example below. 

```rust
use ethers::{
    contract::abigen,
    providers::{Provider, Ws},
};
use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./abi/Erc20Token.abi"); // Path to abi (json)
abirpc!(Erc20Token, Erc20TokenRegistry);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = address_from!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?; // WETH
    let registry = Erc20TokenRegistry::<Provider<Ws>>::new(
    	Some(String::from("wss://ethereum-rpc.publicnode.com")), 
    	Some(Chain::from(NamedChain::Mainnet))
    );
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address);

    let _ = instance.decimals().await?; // Query contract abi

    Ok(())
}
```

Note that the `abigen!` macro generates the rust bindings for the contract ABI, and is required for `abirpc!` to function. 

### ABI management

ABI files can be located anywhere on the system, and multiple ABIs can be initialized within the same `.rs` file. 

```rust
use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./abi/Erc20Token.abi"); 
abirpc!(Erc20Token, Erc20TokenRegistry);

abigen!(Erc721Token, "./abi/Erc721Token.abi"); 
abirpc!(Erc721Token, Erc721TokenRegistry);
```

### Network management

Network implementation is consistent with the [`alloy`](https://github.com/alloy-rs/alloy) API.

```rust
let chain = Chain::from(NamedChain::Mainnet);
// OR
let chain = Chain::Id(1);
```

If the chain `Id` does not match the on-chain configuration, initialization will fail.

```rust
let registry = Erc20TokenRegistry::<Provider<Ws>>::new(
	Some(String::from("wss://ethereum-rpc.publicnode.com")), 
	Some(Chain::Id(10)) // Incorrect ChainId
);
let provider = registry.provider().await?; // Error 
```

Passing a `ChainConfig` provides granular control over all configuration parameters.

```rust 
let chain = Chain::ChainConfig(ChainConfig::default())
```

### Provider management

The crate also includes a wrapper for initialization of supported providers. This is helpful for interactions not requiring an ABI.

```rust
let provider: Provider<Ws> = AbiProvider::new(
    Some(String::from("wss://ethereum-rpc.publicnode.com")),
    Some(Chain::Id(1)),
)
.provider()
.await?;

let mut stream = provider.subscribe_blocks().await?;
while let Some(block) = stream.next().await {
    println!("{:?}", block)
}
```
