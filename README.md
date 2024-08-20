# ethers-abirpc

`ethers-abirpc` allows users to efficiently manage multiple smart contract instances across multiple blockchains within the same application context via a simple API. 

## Overview

The crate defines the `abirpc!` macro along with several other utilities for [`ethers-rs`](https://github.com/gakonst/ethers-rs) provider encapsulation. The `abirpc!` macro is implemented as an extension of the ethers `abigen!` macro as shown in the example below.

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

In this example, the `abirpc!(Erc20Token)` call generates the `Erc20TokenRegistry` type which implements RPC provider encapsulation, and the preceding `abigen!` call generates the underlying `Erc20Token` type which defines the rust bindings for the contract ABI.

## Provider management

`ethers-abirpc` supports the following `ethers-rs` provider types:

```rust
Provider<Ws>
Provider<Http>
Provider<RetryClient<Http>>
Provider<Ipc>
Provider<MockProvider>
```

These types are re-exported by `ethers-abirpc` via the following type aliases so developers do not need to manage underlying `ethers-rs` provider types directly:

```rust
WsProvider
HttpProvider
RetryProvider
IpcProvider
MockProvider
```

The crate also supports direct initialization of providers. This is helpful for applications which do not require ABI interaction.

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

## Network management

Network implementation is consistent with the [`alloy`](https://github.com/alloy-rs/alloy) API.

```rust
let chain = Chain::from(NamedChain::Mainnet);
// OR
let chain = Chain::from_id(1);
```

If the chain id does not match the on-chain configuration, initialization will fail.

```rust
let registry = Erc20TokenRegistry::<WsProvider>::new(
    String::from("wss://ethereum-rpc.publicnode.com"), 
    Chain::from_id(10) // Incorrect ChainId
);
let provider = registry.provider().await?; // Error 
```

Passing a `ChainConfig` provides granular control over all configuration parameters, including the enforcement of chain id checks.

```rust 
let chain = Chain::ChainConfig(ChainConfig::default())
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

- 0.3.0: Improve macros, imports, and add type aliases for provider types
- 0.2.x: Stabilized API and add alloy compatible chain implementations
- 0.1.x: Development versions

An [`alloy-abirpc`](https://crates.io/crates/alloy-abirpc) implementation of the `ethers-abirpc` API has also been developed, and efforts have been made to ensure that both libraries share a consistent API. 
