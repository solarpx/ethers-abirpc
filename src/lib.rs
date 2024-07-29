#![allow(non_snake_case)]
pub mod error;
pub mod network;
pub mod provider;
pub mod registry;

pub mod prelude {
    pub use crate::network::Network;
    pub use crate::provider::{AbiProvider, AbiProviderTrait};
    pub use crate::{abirpc, address_from};
    pub use ethers::contract::abigen;
}
