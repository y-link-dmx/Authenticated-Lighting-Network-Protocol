//! High-level ALPINE SDK built on top of the published protocol bindings.
//! The crate keeps discovery, connection, and streaming lifecycles explicit
//! while favoring a minimal public façade.
pub mod client;
pub mod discovery;
pub mod error;
pub mod transport;

pub use client::AlpineClient;
pub use discovery::{DiscoveryClient, DiscoveryClientOptions, DiscoveryError, DiscoveryOutcome};
pub use error::AlpineSdkError;
pub use transport::{quic::QuicFrameTransport, udp::UdpFrameTransport};
