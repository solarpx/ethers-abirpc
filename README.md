# ethers-abirpc

`ethers-abirpc` allows users to efficiently manage multiple smart contract instances across multiple blockchains within the same application context via a simple API. 

## Overview 

The crate defines the `abirpc!` macro along with several other utilities for [ethers-rs](https://github.com/gakonst/ethers-rs) provider encapsulation. The following ethers-rs provider types are supported:

```rust
Provider<Ws>
Provider<Http>
Provider<RetryClient<Http>>
Provider<Ipc>
Provider<MockProvider>
```

The `abirpc!` macro is implemented as an extension of ethers-rs `abigen!` as shown in the example below.

```rust
use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./abi/Erc20Token.json"); // Path to abi
abirpc!(Erc20Token);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = address_from!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?; // WETH
    let registry = Erc20TokenRegistry::<WsProvider>::new(
    	String::from("wss://ethereum-rpc.publicnode.com"), 
    	Chain::from(NamedChain::Mainnet)
    );
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address);

    let _ = instance.decimals().await?; // Query contract abi

    Ok(())
}
```

Note that the `abigen!` macro generates the rust bindings for the contract ABI, and is required for `abirpc!` to function.

## Network management

Network implementation is consistent with the [alloy](https://github.com/alloy-rs/alloy) API.

```rust
let chain = Chain::from(NamedChain::Mainnet);
// OR
let chain = Chain::Id(1);
```

If the chain `Id` does not match the on-chain configuration, initialization will fail.

```rust
let registry = Erc20TokenRegistry::<WsProvider>::new(
	String::from("wss://ethereum-rpc.publicnode.com"), 
	Chain::Id(10) // Incorrect ChainId
);
let provider = registry.provider().await?; // Error 
```

Passing a `ChainConfig` provides granular control over all configuration parameters, including the enforcement of chain `Id` checks.

```rust 
let chain = Chain::ChainConfig(ChainConfig::default())
```

## Provider management

The crate also includes a wrapper for direct initialization of supported `ethers-rs` provider types. This is helpful for interactions not requiring an ABI.

```rust
let provider: WsProvider = AbiProvider::new(
    String::from("wss://ethereum-rpc.publicnode.com"),
    Chain::Id(1),
)
.provider()
.await?;

let mut stream = provider.subscribe_blocks().await?;
while let Some(block) = stream.next().await {
    println!("{:?}", block)
}
```

## ABI management

ABI files can be located anywhere on the system, and multiple ABIs can be initialized within the same `.rs` file.

```rust
use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./abi/Erc20Token.json"); 
abirpc!(Erc20Token);

abigen!(Erc721Token, "./abi/Erc721Token.json"); 
abirpc!(Erc721Token);
```

## Release notes

- 0.3.0: Improve macro, imports, and add type aliases for provider types
- 0.2.x: Stabilized API and add alloy compatible chain implementations
- 0.1.x: Development versions
