# ethers-abirpc

This library defines the `abirpc!` macro for ethers-rs provider encapsulation. The following ethers-rs provider types are supported 

```rust
Ws
Http
RetryClient<Http>
MockProvider
```

The `abirpc!` macro is implemented as an extension of ethers-rs `abigen!` and auto-imports all required ethers-rs dependencies. 

```rust
use ethers_abirpc::abirpc;
abigen!(Erc20Token, "./tests/abi/Erc20Token.abi"); // Path to abi (json)
abirpc!(Erc20Token, Erc20TokenRegistry);
``` 

`abirpc!` invocation generates a simple API for initialization of smart contract instances, and the underlying `abigen!` provides the rust bindings for the smart contract. 

```rust
use ethers_abirpc::address_from;
let address = address_from!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2") // WETH

let registry = Erc20TokenRegistry::<Http>::new(Some(url), Some(Network::ETHEREUM));
let provider = registry.provider().await?;
let instance = registry.register(provider, address);

let decimals = instance.decimals().await?; // Query contract abi
```
