use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use alpine_protocol_rs::control::{ControlClient, ControlCrypto};
use alpine_protocol_rs::crypto::identity::NodeCredentials;
use alpine_protocol_rs::crypto::X25519KeyExchange;
use alpine_protocol_rs::handshake::keepalive;
use alpine_protocol_rs::handshake::transport::{CborUdpTransport, TimeoutTransport};
use alpine_protocol_rs::handshake::{HandshakeContext, HandshakeError};
use alpine_protocol_rs::messages::{CapabilitySet, ChannelFormat, ControlEnvelope, ControlOp, DeviceIdentity};
use alpine_protocol_rs::profile::StreamProfile;
use alpine_protocol_rs::session::{AlnpSession, Ed25519Authenticator};
use alpine_protocol_rs::stream::AlnpStream;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::error::AlpineSdkError;
use crate::transport::UdpFrameTransport;

/// High-level client that wraps the ALPINE protocol primitives.
#[derive(Debug)]
pub struct AlpineClient {
    session: AlnpSession,
    _transport: Arc<Mutex<TimeoutTransport<CborUdpTransport>>>,
    local_addr: SocketAddr,
    remote_addr: SocketAddr,
    stream: Option<AlnpStream<UdpFrameTransport>>,
    control: ControlClient,
    keepalive_handle: Option<JoinHandle<()>>,
}

impl AlpineClient {
    /// Opens a session with the provided device identity and capabilities.
    pub async fn connect(
        local_addr: SocketAddr,
        remote_addr: SocketAddr,
        identity: DeviceIdentity,
        capabilities: CapabilitySet,
        credentials: NodeCredentials,
    ) -> Result<Self, AlpineSdkError> {
        let key_exchange = X25519KeyExchange::new();
        let authenticator = Ed25519Authenticator::new(credentials.clone());

        let mut transport = TimeoutTransport::new(
            CborUdpTransport::bind(local_addr, remote_addr, 2048).await?,
            Duration::from_secs(3),
        );
        let session = AlnpSession::connect(
            identity,
            capabilities.clone(),
            authenticator,
            key_exchange,
            HandshakeContext::default(),
            &mut transport,
        )
        .await?;

        let transport = Arc::new(Mutex::new(transport));
        let keepalive_handle = tokio::spawn(keepalive::spawn_keepalive(
            transport.clone(),
            Duration::from_secs(5),
            session
                .established()
                .ok_or_else(|| AlpineSdkError::Io("session missing after handshake".into()))?
                .session_id,
        ));

        let established = session
            .established()
            .ok_or_else(|| AlpineSdkError::Io("session missing after handshake".into()))?;
        let device_uuid = Uuid::parse_str(&established.device_identity.device_id)
            .unwrap_or_else(|_| Uuid::new_v4());
        let control_crypto = ControlCrypto::new(
            session
                .keys()
                .ok_or_else(|| AlpineSdkError::Io("session keys missing".into()))?,
        );
        let control = ControlClient::new(device_uuid, established.session_id, control_crypto);

        Ok(Self {
            session,
            _transport: transport,
            local_addr,
            remote_addr,
            stream: None,
            control,
            keepalive_handle: Some(keepalive_handle),
        })
    }

    /// Starts streaming with the supplied profile and returns the generated config id.
    pub fn start_stream(&mut self, profile: StreamProfile) -> Result<String, AlpineSdkError> {
        let compiled = profile
            .compile()
            .map_err(|err| HandshakeError::Protocol(err.to_string()))?;
        self.session
            .set_stream_profile(compiled.clone())
            .map_err(AlpineSdkError::Handshake)?;
        self.session.mark_streaming();

        let stream_socket = UdpFrameTransport::new(self.local_addr, self.remote_addr)?;
        let stream = AlnpStream::new(self.session.clone(), stream_socket, compiled.clone());
        self.stream = Some(stream);
        Ok(compiled.config_id().to_string())
    }

    /// Sends a streaming frame over the active session.
    pub fn send_frame(
        &self,
        channel_format: ChannelFormat,
        channels: Vec<u16>,
        priority: u8,
        groups: Option<HashMap<String, Vec<u16>>>,
        metadata: Option<HashMap<String, Value>>,
    ) -> Result<(), AlpineSdkError> {
        let stream = self
            .stream
            .as_ref()
            .ok_or_else(|| AlpineSdkError::Io("stream not started".into()))?;
        stream
            .send(channel_format, channels, priority, groups, metadata)
            .map_err(AlpineSdkError::from)
    }

    /// Stops keep-alive and shuts down the session.
    pub async fn close(mut self) {
        self.session.close();
        if let Some(handle) = self.keepalive_handle.take() {
            handle.abort();
        }
    }

    /// Builds a signed control envelope for the active session.
    pub fn control_envelope(
        &self,
        seq: u64,
        op: ControlOp,
        payload: Value,
    ) -> Result<ControlEnvelope, HandshakeError> {
        self.control.envelope(seq, op, payload)
    }
}
