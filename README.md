# ethers-abirpc

This library defines the `abirpc!` macro along with several other utilites for ethers-rs provider encapsulation. The following ethers-rs provider types are supported

```rust
Provider<Ws>
Provider<Ipc>
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

ABIs inclusion occurs at compile time. ABI files can be located anywhere on the system, and multiple ABIs can be initialized within the same `.rs` file. 

```rust
use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./abi/Erc20Token.abi"); 
abirpc!(Erc20Token, Erc20TokenRegistry);

abigen!(Erc677Token, "./abi/Erc677Token.abi"); 
abirpc!(Erc677Token, Erc677TokenRegistry);

abigen!(Erc721Token, "./abi/Erc721Token.abi"); 
abirpc!(Erc721Token, Erc721TokenRegistry);
```

### Network management

Network initialization can be achieved via keyword or by specifying the desired `ChainId` 

```rust 
let network = Network::ETHEREUM;
// OR
let network = Network::ChainId(1);
```

Whenever an ethers-rs provider is constructed in `ethers_abigen`, the `ChainId` is validated by querying the on-chain configuration. If the `ChainId`s do not match, initialization will fail. 

```rust
let registry = Erc20TokenRegistry::<Provider<Http>>::new(
	Some(String::from("wss://ethereum-rpc.publicnode.com")), 
	Some(Network::ChainId(10)) // Incorrect ChainId
);
let provider = registry.provider().await?; // Error 
```

### Provider management

The crate also includes a wrapper for initialization of supported ethers-rs providers. This is helpful for interacting with ethers-rs primitives outside the context of smart contract interaction.

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

