/// This crate provides the network server for Lighthouse.
pub mod error;
pub mod service;

mod router;
mod shard_manager;
mod sync;

pub use eth2_libp2p::NetworkConfig;
pub use router::processor::Processor;
pub use service::NetworkMessage;
pub use service::Service;
