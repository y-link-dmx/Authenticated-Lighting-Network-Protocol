use async_trait::async_trait;
use rand::{rngs::OsRng, RngCore};
use thiserror::Error;

use crate::crypto::{KeyExchange, KeyExchangeAlgorithm};
use crate::messages::{
    ChallengeRequest, ChallengeResponse, ControllerHello, NodeHello, SessionEstablished,
};

pub mod client;
pub mod server;

/// Transport abstraction used during the ALNP handshake.
#[async_trait]
pub trait HandshakeTransport {
    async fn send(&mut self, msg: HandshakeMessage) -> Result<(), HandshakeError>;
    async fn recv(&mut self) -> Result<HandshakeMessage, HandshakeError>;
}

/// Minimal message envelope for the handshake pipeline.
#[derive(Debug, Clone, PartialEq)]
pub enum HandshakeMessage {
    ControllerHello(ControllerHello),
    NodeHello(NodeHello),
    ChallengeRequest(ChallengeRequest),
    ChallengeResponse(ChallengeResponse),
    SessionEstablished(SessionEstablished),
}

/// Context shared between handshake participants.
#[derive(Debug, Clone)]
pub struct HandshakeContext {
    pub key_algorithm: KeyExchangeAlgorithm,
    pub expected_controller: Option<String>,
    pub required_firmware_rev: Option<String>,
}

impl Default for HandshakeContext {
    fn default() -> Self {
        Self {
            key_algorithm: KeyExchangeAlgorithm::X25519,
            expected_controller: None,
            required_firmware_rev: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("protocol violation: {0}")]
    Protocol(String),
    #[error("authentication failed: {0}")]
    Authentication(String),
    #[error("unsupported capability: {0}")]
    Capability(String),
}

/// Generates a cryptographic nonce for challenge/response.
pub fn new_nonce() -> [u8; 32] {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

/// Shared behavior between controller and node handshake roles.
#[async_trait]
pub trait HandshakeParticipant {
    async fn run<T: HandshakeTransport + Send>(&self, transport: &mut T)
        -> Result<SessionEstablished, HandshakeError>;
}

/// Minimal authenticator stub for challenge validation.
pub trait ChallengeAuthenticator {
    fn sign_challenge(&self, nonce: &[u8]) -> Vec<u8>;
    fn verify_challenge(&self, nonce: &[u8], signature: &[u8]) -> bool;
}
