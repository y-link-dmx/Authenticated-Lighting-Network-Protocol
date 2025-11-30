use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use alpine::control::{ControlClient, ControlCrypto};
use alpine::crypto::identity::NodeCredentials;
use alpine::crypto::X25519KeyExchange;
use alpine::handshake::keepalive;
use alpine::handshake::transport::{CborUdpTransport, ReliableControlChannel, TimeoutTransport};
use alpine::handshake::{HandshakeContext, HandshakeError, HandshakeMessage, HandshakeTransport};
use alpine::messages::{
    Acknowledge, CapabilitySet, ChannelFormat, ControlEnvelope, ControlOp, DeviceIdentity,
};
use alpine::profile::StreamProfile;
use alpine::session::{AlnpSession, Ed25519Authenticator};
use alpine::stream::AlnpStream;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
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

/// Typed control response returned by `AlpineClient` helpers.
#[derive(Debug)]
pub struct ControlReply<T> {
    pub ack: Acknowledge,
    pub payload: Option<T>,
}

impl<T> ControlReply<T> {
    pub fn ok(&self) -> bool {
        self.ack.ok
    }

    pub fn detail(&self) -> Option<&str> {
        self.ack.detail.as_deref()
    }
}

/// Ping reply payload (may be partial depending on device support).
#[derive(Debug, Deserialize)]
pub struct PingReply {
    #[serde(default)]
    pub timestamp_ms: Option<u64>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Status reply payload returned by the `status` helper.
#[derive(Debug, Deserialize)]
pub struct StatusReply {
    #[serde(default)]
    pub healthy: Option<bool>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub uptime_secs: Option<u64>,
}

/// Health reply payload, including optional metrics metadata.
#[derive(Debug, Deserialize)]
pub struct HealthReply {
    #[serde(default)]
    pub healthy: Option<bool>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub metrics: Option<HashMap<String, Value>>,
}

/// Alias for the fetched device identity.
pub type IdentityReply = DeviceIdentity;

/// Metadata reply payload is an arbitrary map of CBOR values.
#[derive(Debug, Deserialize)]
pub struct MetadataReply {
    #[serde(default)]
    pub metadata: HashMap<String, Value>,
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

    /// Sends a ping command and returns the parsed reply (CBOR payload optional).
    pub async fn ping(&self) -> Result<ControlReply<PingReply>, AlpineSdkError> {
        self.control_command("ping").await
    }

    /// Returns the status payload the node publishes for callers.
    pub async fn status(&self) -> Result<ControlReply<StatusReply>, AlpineSdkError> {
        self.control_command("status").await
    }

    /// Reads the health payload, including optional metrics.
    pub async fn health(&self) -> Result<ControlReply<HealthReply>, AlpineSdkError> {
        self.control_command("health").await
    }

    /// Requests the device identity through the control channel.
    pub async fn identity(&self) -> Result<ControlReply<IdentityReply>, AlpineSdkError> {
        self.control_command("identity").await
    }

    /// Fetches metadata that the device publishes in CBOR.
    pub async fn metadata(&self) -> Result<ControlReply<MetadataReply>, AlpineSdkError> {
        self.control_command("metadata").await
    }

    async fn control_command<T>(&self, command: &str) -> Result<ControlReply<T>, AlpineSdkError>
    where
        T: DeserializeOwned,
    {
        let payload = json!({ "command": command });
        self.control_request(ControlOp::Vendor, payload).await
    }

    async fn control_request<T>(
        &self,
        op: ControlOp,
        payload: Value,
    ) -> Result<ControlReply<T>, AlpineSdkError>
    where
        T: DeserializeOwned,
    {
        let transport = SharedTransport::new(self._transport.clone());
        let mut channel = ReliableControlChannel::new(transport);
        let ack = self.control.send(&mut channel, op, payload).await?;
        let parsed = ControlCrypto::decode_ack_payload::<T>(ack.payload.as_deref())
            .map_err(AlpineSdkError::from)?;
        Ok(ControlReply {
            ack,
            payload: parsed,
        })
    }
}

#[derive(Clone)]
struct SharedTransport<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> SharedTransport<T> {
    fn new(inner: Arc<Mutex<T>>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<T> HandshakeTransport for SharedTransport<T>
where
    T: HandshakeTransport + Send,
{
    async fn send(&mut self, msg: HandshakeMessage) -> Result<(), HandshakeError> {
        let mut guard = self.inner.lock().await;
        guard.send(msg).await
    }

    async fn recv(&mut self) -> Result<HandshakeMessage, HandshakeError> {
        let mut guard = self.inner.lock().await;
        guard.recv().await
    }
}
