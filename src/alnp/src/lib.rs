//! Authenticated Lighting Network Protocol (ALNP) scaffolding.
//!
//! This crate layers an authenticated control plane over the existing sACN
//! transport while keeping packet formats untouched. The streaming path is
//! guarded by a handshake derived from ESTA E1.33 patterns.

pub mod crypto;
pub mod handshake;
pub mod messages;
pub mod session;
pub mod stream;
pub mod control;

pub use session::{AlnpRole, AlnpSession, JitterStrategy};
pub use messages::{
    Acknowledge, CapabilitySet, ControlEnvelope, ControlHeader, ControlPayload, DeviceIdentity,
    OperatingMode as AlnpOperatingMode,
};

/// Public device info provided to higher-level APIs.
pub type AlnpDeviceInfo = messages::DeviceInfo;
/// Public capability descriptor.
pub type AlnpCapabilities = messages::CapabilitySet;

/// Control-plane client facade.
pub struct AlnpControlClient {
    pub session: AlnpSession,
}

impl AlnpControlClient {
    pub fn new(session: AlnpSession) -> Self {
        Self { session }
    }

    pub async fn send_control<T: handshake::HandshakeTransport + Send>(
        &self,
        channel: &mut handshake::transport::ReliableControlChannel<T>,
        envelope: ControlEnvelope,
    ) -> Result<Acknowledge, handshake::HandshakeError> {
        channel.send_reliable(envelope).await
    }
}

/// Streaming client wrapper.
pub struct AlnpStreamingClient<T: stream::SacnStreamAdapter> {
    pub stream: stream::AlnpStream<T>,
}

impl<T: stream::SacnStreamAdapter> AlnpStreamingClient<T> {
    pub fn new(session: AlnpSession, sacn: T) -> Self {
        Self {
            stream: stream::AlnpStream::new(session, sacn),
        }
    }

    pub fn stream(&self) -> &stream::AlnpStream<T> {
        &self.stream
    }
}
