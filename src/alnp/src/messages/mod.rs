use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
