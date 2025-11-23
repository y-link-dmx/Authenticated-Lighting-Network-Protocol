use rand::{rngs::OsRng, RngCore};

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
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

impl X25519KeyExchange {
    pub fn new() -> Self {
        // Random bytes serve as stand-ins for a real keypair while keeping the API stable.
        let mut public_key = vec![0u8; 32];
        let mut private_key = vec![0u8; 32];
        OsRng.fill_bytes(&mut public_key);
        OsRng.fill_bytes(&mut private_key);
        Self {
            public_key,
            private_key,
        }
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
        self.public_key.clone()
    }

    fn derive_shared(&self, peer_public_key: &[u8]) -> SessionKeys {
        // Placeholder: concatenate keys to obtain deterministic material for higher-level hashing.
        let mut shared = Vec::with_capacity(self.private_key.len() + peer_public_key.len());
        shared.extend_from_slice(&self.private_key);
        shared.extend_from_slice(peer_public_key);
        SessionKeys {
            shared_secret: shared,
            stream_key: None,
        }
    }
}

/// Interface that would wrap an external TLS channel when available.
pub trait TlsWrapper {
    fn wrap_stream(&self, plaintext: &[u8]) -> Vec<u8>;
    fn unwrap_stream(&self, ciphertext: &[u8]) -> Vec<u8>;
}
