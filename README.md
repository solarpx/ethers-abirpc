# ethers-abirpc

This crate defines the `abirpc!` macro along with several other utilites for ethers-rs provider encapsulation. The following ethers-rs provider types are supported:

```rust
Provider<Ws>
Provider<Http>
Provider<RetryClient<Http>>
Provider<Ipc>
Provider<MockProvider>
```

The `abirpc!` macro is implemented as an extension of ethers-rs `abigen!`. `abirpc!` generates an API to manage deployed contract instances, and `abigen!` provides the rust bindings for the contract ABI. 

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
    	Some(Network::ETHEREUM)
    );
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address);

    let _ = instance.decimals().await?; // Query contract abi

    Ok(())
}
```

### ABI management

ABI inclusion occurs at compile time. ABI files can be located anywhere on the system, and multiple ABIs can be initialized within the same `.rs` file. 

```rust
use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./abi/Erc20Token.abi"); 
abirpc!(Erc20Token, Erc20TokenRegistry);

abigen!(Erc721Token, "./abi/Erc721Token.abi"); 
abirpc!(Erc721Token, Erc721TokenRegistry);
```

### Network management

Network initialization is achieved by specifying the desired `ChainId`. Whenever a provider is constructed, its `ChainId` is validated by querying the on-chain configuration.

```rust 
let network = Network::ChainId(1);
```

 If the initlaized `ChainId` does not match the on-chain configuration, initialization will fail. 

```rust
let registry = Erc20TokenRegistry::<Provider<Ws>>::new(
	Some(String::from("wss://ethereum-rpc.publicnode.com")), 
	Some(Network::ChainId(10)) // Incorrect ChainId
);
let provider = registry.provider().await?; // Error 
```

It is also possible to configure a network by passing a `NetworkConfig`. This provides granular control over all parameters. In the default case, `ChainId` is set to `None` bypassing on-chain validation.

```rust 
let network = Network::NetworkConfig(NetworkConfig::default())
```

### Provider management

The crate also includes a wrapper for initialization of all supported providers. This is helpful for interacting with ethers-rs primitives outside the context of smart contract interaction.

```rust
let provider: Provider<Ws> = AbiProvider::new(
    Some(String::from("wss://ethereum-rpc.publicnode.com")),
    Some(Network::ChainId(1)),
)
.provider()
.await?;

let mut stream = provider.subscribe_blocks().await?;
while let Some(block) = stream.next().await {
    println!("{:?}", block)
}
```

### Release notes

Release versions

`v0.2.0`: stable release

Development versions

`v0.1.3`: stable release

`v0.1.2`: unstable imports

`v0.1.1`: unsupported

`v0.1.0`: unsupported
