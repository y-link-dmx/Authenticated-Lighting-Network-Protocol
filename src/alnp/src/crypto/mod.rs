use rand::rngs::OsRng;
use x25519_dalek::{PublicKey as X25519PublicKey, SharedSecret, StaticSecret as X25519Secret};

pub mod identity;

/// Algorithms supported for the initial key exchange.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyExchangeAlgorithm {
    X25519,
    EcdhP256,
    None,
}

/// Derived session key material.
#[derive(Debug, Clone)]
pub struct SessionKeys {
    pub shared_secret: Vec<u8>,
    pub stream_key: Option<Vec<u8>>,
}

/// Behavior required to complete the handshake key agreement.
pub trait KeyExchange {
    fn algorithm(&self) -> KeyExchangeAlgorithm;
    fn public_key(&self) -> Vec<u8>;
    fn derive_shared(&self, peer_public_key: &[u8]) -> SessionKeys;
}

/// Lightweight placeholder for X25519; replace with a real implementation later.
pub struct X25519KeyExchange {
    public_key: X25519PublicKey,
    private_key: X25519Secret,
}

impl X25519KeyExchange {
    pub fn new() -> Self {
        let private_key = X25519Secret::random_from_rng(OsRng);
        let public_key = X25519PublicKey::from(&private_key);
        Self { public_key, private_key }
    }
}

impl Default for X25519KeyExchange {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyExchange for X25519KeyExchange {
    fn algorithm(&self) -> KeyExchangeAlgorithm {
        KeyExchangeAlgorithm::X25519
    }

    fn public_key(&self) -> Vec<u8> {
        self.public_key.to_bytes().to_vec()
    }

    fn derive_shared(&self, peer_public_key: &[u8]) -> SessionKeys {
        let peer_bytes: [u8; 32] = peer_public_key
            .try_into()
            .expect("peer public key must be 32 bytes");
        let peer_pk = X25519PublicKey::from(peer_bytes);
        let shared_secret: SharedSecret = self.private_key.diffie_hellman(&peer_pk);
        let shared_secret = shared_secret.as_bytes().to_vec();
        SessionKeys {
            shared_secret,
            stream_key: None,
        }
    }
}

/// Interface that would wrap an external TLS channel when available.
pub trait TlsWrapper {
    fn wrap_stream(&self, plaintext: &[u8]) -> Vec<u8>;
    fn unwrap_stream(&self, ciphertext: &[u8]) -> Vec<u8>;
}
