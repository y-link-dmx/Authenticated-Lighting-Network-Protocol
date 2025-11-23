use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Standard control-plane header for replay protection and ordering.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlHeader {
    pub seq: u64,
    pub nonce: Vec<u8>,
    pub timestamp_ms: u64,
}

/// Signed acknowledge wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Acknowledge {
    pub header: ControlHeader,
    pub ok: bool,
    pub detail: Option<String>,
    pub signature: Vec<u8>,
}

/// Envelope that carries a control payload plus signature for replay protection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlEnvelope {
    pub header: ControlHeader,
    pub payload: ControlPayload,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "body")]
pub enum ControlPayload {
    Identify(IdentifyRequest),
    IdentifyResponse(IdentifyResponse),
    GetDeviceInfo(DeviceInfoRequest),
    DeviceInfo(DeviceInfo),
    GetCapabilities,
    Capabilities(CapabilitySet),
    SetWifiCreds(SetWifiCreds),
    SetUniverseMapping(SetUniverseMapping),
    SetMode(SetMode),
    GetStatus,
    StatusReport(StatusReport),
    Restart(RestartRequest),
}

/// ALNP protocol semantic version.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl ProtocolVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }

    pub const fn alnp_v1() -> Self {
        Self::new(1, 0, 0)
    }
}

/// Device identity tuple exchanged during handshake.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceIdentity {
    pub cid: Uuid,
    pub manufacturer: String,
    pub model: String,
    pub firmware_rev: String,
}

/// Declared capabilities modeled loosely after RDMnet client connect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilitySet {
    pub supports_encryption: bool,
    pub supports_redundancy: bool,
    pub max_universes: Option<u16>,
    pub vendor_data: Option<String>,
}

/// Controller -> Node hello message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControllerHello {
    pub controller: DeviceIdentity,
    pub requested_version: ProtocolVersion,
    pub capabilities: CapabilitySet,
    pub key_exchange: KeyExchangeProposal,
}

/// Node -> Controller hello message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeHello {
    pub node: DeviceIdentity,
    pub supported_version: ProtocolVersion,
    pub capabilities: CapabilitySet,
    pub key_exchange: KeyExchangeProposal,
    pub auth_required: bool,
}

/// Challenge issued by node to controller.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChallengeRequest {
    pub nonce: Vec<u8>,
    pub controller_expected: Uuid,
    pub signature_scheme: SignatureScheme,
}

/// Response from controller to prove identity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChallengeResponse {
    pub nonce: Vec<u8>,
    pub signature: Vec<u8>,
    pub key_confirmation: Option<Vec<u8>>,
}

/// Session ready marker.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionEstablished {
    pub session_id: Uuid,
    pub agreed_version: ProtocolVersion,
    pub stream_key: Option<Vec<u8>>,
    pub expires_at_epoch_ms: Option<u64>,
}

/// Control-plane keepalive frame to detect dead sessions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Keepalive {
    pub session_id: Option<Uuid>,
    pub tick_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdentifyRequest {
    pub blink: bool,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdentifyResponse {
    pub acknowledged: bool,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceInfoRequest;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceInfo {
    pub identity: DeviceIdentity,
    pub version: ProtocolVersion,
    pub mode: OperatingMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetWifiCreds {
    pub ssid: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniverseMapping {
    pub universe: u16,
    pub output_port: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetUniverseMapping {
    pub mappings: Vec<UniverseMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperatingMode {
    Normal,
    Calibration,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetMode {
    pub mode: OperatingMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusReport {
    pub healthy: bool,
    pub detail: Option<String>,
    pub uptime_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RestartRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyExchangeProposal {
    pub algorithm: String,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignatureScheme {
    Ed25519,
    EcdsaP256,
}
