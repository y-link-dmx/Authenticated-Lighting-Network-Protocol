use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::crypto::{KeyExchange, X25519KeyExchange};
use crate::handshake::{
    client::ClientHandshake, server::ServerHandshake, ChallengeAuthenticator, HandshakeContext,
    HandshakeError, HandshakeParticipant, HandshakeTransport,
};
use crate::messages::{CapabilitySet, DeviceIdentity, ProtocolVersion, SessionEstablished};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlnpRole {
    Controller,
    Node,
}

#[derive(Debug, Clone)]
pub struct AlnpSession {
    pub role: AlnpRole,
    inner: Arc<Mutex<SessionState>>,
}

#[derive(Debug, Clone)]
enum SessionState {
    Initialized,
    Established(SessionEstablished),
    Failed(String),
}

impl AlnpSession {
    pub fn new(role: AlnpRole) -> Self {
        Self {
            role,
            inner: Arc::new(Mutex::new(SessionState::Initialized)),
        }
    }

    pub fn established(&self) -> Option<SessionEstablished> {
        let guard = self.inner.lock().ok()?;
        match &*guard {
            SessionState::Established(sess) => Some(sess.clone()),
            _ => None,
        }
    }

    pub fn ensure_established(&self) -> Result<SessionEstablished, HandshakeError> {
        self.established().ok_or_else(|| {
            HandshakeError::Authentication("session not established; streaming blocked".into())
        })
    }

    pub async fn connect<T, A, K>(
        identity: DeviceIdentity,
        capabilities: CapabilitySet,
        protocol_version: ProtocolVersion,
        authenticator: A,
        key_exchange: K,
        context: HandshakeContext,
        transport: &mut T,
    ) -> Result<Self, HandshakeError>
    where
        T: HandshakeTransport + Send,
        A: ChallengeAuthenticator + Send + Sync,
        K: KeyExchange + Send + Sync,
    {
        let session = Self::new(AlnpRole::Controller);
        let driver = ClientHandshake {
            identity,
            capabilities,
            protocol_version,
            authenticator,
            key_exchange,
            context,
        };

        let established = driver.run(transport).await?;
        if let Ok(mut guard) = session.inner.lock() {
            *guard = SessionState::Established(established);
        }
        Ok(session)
    }

    pub async fn accept<T, A, K>(
        identity: DeviceIdentity,
        capabilities: CapabilitySet,
        protocol_version: ProtocolVersion,
        authenticator: A,
        key_exchange: K,
        context: HandshakeContext,
        transport: &mut T,
    ) -> Result<Self, HandshakeError>
    where
        T: HandshakeTransport + Send,
        A: ChallengeAuthenticator + Send + Sync,
        K: KeyExchange + Send + Sync,
    {
        let session = Self::new(AlnpRole::Node);
        let driver = ServerHandshake {
            identity,
            capabilities,
            protocol_version,
            authenticator,
            key_exchange,
            context,
        };

        let established = driver.run(transport).await?;
        if let Ok(mut guard) = session.inner.lock() {
            *guard = SessionState::Established(established);
        }
        Ok(session)
    }
}

/// Shared-secret authenticator placeholder for signing and verification.
pub struct StaticKeyAuthenticator {
    secret: Vec<u8>,
}

impl StaticKeyAuthenticator {
    pub fn new(secret: Vec<u8>) -> Self {
        Self { secret }
    }
}

impl Default for StaticKeyAuthenticator {
    fn default() -> Self {
        Self::new(b"default-alnp-secret".to_vec())
    }
}

impl ChallengeAuthenticator for StaticKeyAuthenticator {
    fn sign_challenge(&self, nonce: &[u8]) -> Vec<u8> {
        let mut sig = Vec::with_capacity(self.secret.len() + nonce.len());
        sig.extend_from_slice(&self.secret);
        sig.extend_from_slice(nonce);
        sig
    }

    fn verify_challenge(&self, nonce: &[u8], signature: &[u8]) -> bool {
        signature.ends_with(nonce) && signature.starts_with(&self.secret)
    }
}

/// Simplified in-memory transport useful for unit tests and examples.
pub struct LoopbackTransport {
    inbox: Vec<crate::handshake::HandshakeMessage>,
}

impl LoopbackTransport {
    pub fn new() -> Self {
        Self { inbox: Vec::new() }
    }
}

#[async_trait]
impl HandshakeTransport for LoopbackTransport {
    async fn send(
        &mut self,
        msg: crate::handshake::HandshakeMessage,
    ) -> Result<(), HandshakeError> {
        self.inbox.push(msg);
        Ok(())
    }

    async fn recv(&mut self) -> Result<crate::handshake::HandshakeMessage, HandshakeError> {
        if self.inbox.is_empty() {
            return Err(HandshakeError::Transport("loopback queue empty".into()));
        }
        Ok(self.inbox.remove(0))
    }
}

/// Helper builder to quickly create a controller-side session with defaults.
pub async fn example_controller_session<T: HandshakeTransport + Send>(
    identity: DeviceIdentity,
    transport: &mut T,
) -> Result<AlnpSession, HandshakeError> {
    AlnpSession::connect(
        identity,
        CapabilitySet {
            supports_encryption: true,
            supports_redundancy: false,
            max_universes: Some(16),
            vendor_data: None,
        },
        ProtocolVersion::alnp_v1(),
        StaticKeyAuthenticator::default(),
        X25519KeyExchange::new(),
        HandshakeContext::default(),
        transport,
    )
    .await
}

/// Helper builder to quickly create a node-side session with defaults.
pub async fn example_node_session<T: HandshakeTransport + Send>(
    identity: DeviceIdentity,
    transport: &mut T,
) -> Result<AlnpSession, HandshakeError> {
    AlnpSession::accept(
        identity,
        CapabilitySet {
            supports_encryption: true,
            supports_redundancy: true,
            max_universes: Some(512),
            vendor_data: None,
        },
        ProtocolVersion::alnp_v1(),
        StaticKeyAuthenticator::default(),
        X25519KeyExchange::new(),
        HandshakeContext::default(),
        transport,
    )
    .await
}
