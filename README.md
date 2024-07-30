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

abigen!(Erc20Token, "./abi/Erc20Token.abi"); // Path to abi (json)
abirpc!(Erc20Token, Erc20TokenRegistry);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = address_from!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?; // WETH
    let url = Url::parse("wss://ethereum-rpc.publicnode.com")?;

    let registry = Erc20TokenRegistry::<Provider<Ws>>::new(
    	Some(url), 
    	Some(Network::ETHEREUM)
    );
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address);

    let _ = instance.decimals().await?; // Query contract abi

    Ok(())
}
```

### Provider management

The crate also includes a wrapper for initialization of supported ethers-rs providers. This is helpful for interacting with ethers-rs primitives. 

```rust
let url = Url::parse("wss://ethereum-rpc.publicnode.com")?;
let provider: Provider<Ws> = AbiProvider::new(Some(url), Some(Network::ChainId(1)))
    .provider()
    .await?;

let mut stream = provider.subscribe_blocks().await?;
while let Some(block) = stream.next().await {
	println!("{:?}", block)
}
```

### Network management

Network initialization can be provided by keyword or by specifying `ChainId` 

```rust 
let ethereum = Network::ETHEREUM;
let network = Network::ChainId(1);
```

Whenever an ethers-rs provider is constructed in the crate context, the `ChainId` is validated by querying for the on-chain configuration. If the `ChainId` does not match the provided network, initialization will fail. 

```rust
let url = Url::parse(TEST_HTTP_PROVIDER)?;
let registry = Erc20TokenRegistry::<Provider<Http>>::new(
	Some("wss://ethereum-rpc.publicnode.com"), 
	Network::ChainId(10) // Incorrect ChainId
);
let provider = registry.provider().await?; // Error 
```

### ABI management

ABIs inclusion occurs at compile time, and files can be located anywhere within your crate. Also, multiple ABIs can be initialized in the same file. 

```rust
use ethers_abirpc::prelude::*;

abigen!(Erc20Token, "./abi/Erc20Token.abi"); 
abirpc!(Erc20Token, Erc20TokenRegistry);

abigen!(Erc677Token, "./abi/Erc677Token.abi"); 
abirpc!(Erc677Token, Erc677TokenRegistry);

abigen!(Erc721Token, "./abi/Erc721Token.abi"); 
abirpc!(Erc721Token, Erc721TokenRegistry);
```

This allows users to build complex interactions with multiple smart contracts and provider types all within the same crate.
