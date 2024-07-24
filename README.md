# ethers-abirpc

This library defines the `abirpc!` macro for ethers-rs provider encapsulation. The following ethers-rs provider types are supported 

```rust
Ws
Http
RetryClient<Http>
MockProvider
```

The `abirpc!` macro is implemented as an extension of ethers-rs `abigen!`

```rust
abigen!(Erc20Token, "./src/abi/token/Erc20Token.abi");
abirpc!(Erc20Token, Erc20TokenRegistry);
``` 

And smart contract interaction is achieved via the `abirpc!` registry abstraction

```rust
let registry = Erc20TokenRegistry::<Http>::new(Some(url), Some(Network::ETHEREUM));
let provider = registry.provider().await?;
let instance = registry.register(provider, address);
let decimals = instance.decimals().await?; // query contract at address
```
