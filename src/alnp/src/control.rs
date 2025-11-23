use std::time::{SystemTime, UNIX_EPOCH};

use crate::handshake::HandshakeError;
use crate::messages::{
    Acknowledge, ControlEnvelope, ControlHeader, ControlPayload, DeviceIdentity,
};
use crate::{handshake::transport::ReliableControlChannel, handshake::HandshakeTransport};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::{rngs::OsRng, RngCore};

/// Signs and verifies control envelopes.
pub struct ControlCrypto {
    signing: SigningKey,
    verifying: VerifyingKey,
    peer_verifying: Option<VerifyingKey>,
}

impl ControlCrypto {
    pub fn new(signing: SigningKey, peer_verifying: Option<VerifyingKey>) -> Self {
        let verifying = signing.verifying_key();
        Self {
            signing,
            verifying,
            peer_verifying,
        }
    }

    pub fn sign_envelope(&self, mut env: ControlEnvelope) -> ControlEnvelope {
        let bytes = serde_json::to_vec(&env.payload).expect("payload serialize");
        let sig = self.signing.sign(&bytes);
        env.signature = sig.to_vec();
        env
    }

    pub fn verify_envelope(&self, env: &ControlEnvelope) -> Result<(), HandshakeError> {
        let bytes = serde_json::to_vec(&env.payload)
            .map_err(|e| HandshakeError::Protocol(format!("encode: {}", e)))?;
        let verifier = self
            .peer_verifying
            .as_ref()
            .unwrap_or(&self.verifying);
        let sig = ed25519_dalek::Signature::from_slice(&env.signature)
            .map_err(|e| HandshakeError::Authentication(e.to_string()))?;
        verifier
            .verify(&bytes, &sig)
            .map_err(|e| HandshakeError::Authentication(e.to_string()))
    }

    pub fn sign_ack(&self, ack: Acknowledge) -> Acknowledge {
        let bytes = serde_json::to_vec(&ack.header).expect("header serialize");
        let sig = self.signing.sign(&bytes);
        Acknowledge {
            signature: sig.to_vec(),
            ..ack
        }
    }
}

/// Control-plane client helper to build signed envelopes and handle acks.
pub struct ControlClient {
    pub device: DeviceIdentity,
    pub crypto: ControlCrypto,
}

impl ControlClient {
    pub fn new(device: DeviceIdentity, crypto: ControlCrypto) -> Self {
        Self { device, crypto }
    }

    pub fn envelope(&self, seq: u64, payload: ControlPayload) -> ControlEnvelope {
        let nonce = Self::nonce();
        let header = ControlHeader {
            seq,
            nonce,
            timestamp_ms: Self::now_ms(),
        };
        self.crypto.sign_envelope(ControlEnvelope {
            header,
            payload,
            signature: vec![],
        })
    }

    pub async fn send<T: HandshakeTransport + Send>(
        &self,
        channel: &mut ReliableControlChannel<T>,
        payload: ControlPayload,
    ) -> Result<Acknowledge, HandshakeError> {
        let seq = channel.next_seq();
        let env = self.envelope(seq, payload);
        channel.send_signed(env).await
    }

    fn nonce() -> Vec<u8> {
        let mut buf = [0u8; 16];
        OsRng.fill_bytes(&mut buf);
        buf.to_vec()
    }

    fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

/// Control responder to validate envelopes and generate signed acks.
pub struct ControlResponder {
    pub device: DeviceIdentity,
    pub crypto: ControlCrypto,
}

impl ControlResponder {
    pub fn new(device: DeviceIdentity, crypto: ControlCrypto) -> Self {
        Self { device, crypto }
    }

    pub fn verify(&self, env: &ControlEnvelope) -> Result<(), HandshakeError> {
        self.crypto.verify_envelope(env)
    }

    pub fn ack(&self, header: ControlHeader, ok: bool, detail: Option<String>) -> Acknowledge {
        self.crypto.sign_ack(Acknowledge {
            header,
            ok,
            detail,
            signature: vec![],
        })
    }
}
