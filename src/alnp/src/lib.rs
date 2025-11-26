//! Authenticated Lighting Network Protocol (ALPINE) reference implementation (v1.0).
//!
//! Implements discovery, handshake, control, and streaming layers as defined in the
//! specification documents. All messages are encoded using CBOR and cryptographically
//! authenticated with Ed25519 + X25519 + HKDF + ChaCha20-Poly1305.

pub mod control;
pub mod crypto;
pub mod discovery;
pub mod handshake;
pub mod messages;
pub mod session;
pub mod stream;
pub mod device;

pub use control::{ControlClient, ControlCrypto, ControlResponder};
pub use messages::{
    Acknowledge, CapabilitySet, ChannelFormat, ControlEnvelope, ControlOp, DeviceIdentity,
    DiscoveryReply, DiscoveryRequest, FrameEnvelope, MessageType, SessionEstablished,
};
pub use session::{AlnpRole, AlnpSession, JitterStrategy};
pub use stream::{AlnpStream, FrameTransport};
pub use device::DeviceServer;
