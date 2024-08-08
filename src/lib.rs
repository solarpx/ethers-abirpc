pub mod error;
pub mod named;
pub mod network;
pub mod provider;
pub mod registry;

pub mod prelude {
    pub use crate::{
        abirpc, address_from,
        error::Error,
        named::NamedChain,
        network::{Network, NetworkConfig, RetryClientConfig},
        provider::{AbiProvider, AbiProviderTrait},
    };
}
