#![allow(clippy::unwrap_used)]
use super::support::MockUdp;
use std::sync::Arc;
use alnp::handshake::{HandshakeParticipant, HandshakeContext};
use alnp::handshake::client::ClientHandshake;
use alnp::handshake::server::ServerHandshake;
use alnp::session::{Ed25519Authenticator, StaticKeyAuthenticator};
use alnp::messages::{CapabilitySet, DeviceIdentity, ProtocolVersion};
use alnp::crypto::{X25519KeyExchange, identity::NodeCredentials};
use super::common::MockHandshakeTransport;

#[tokio::test]
async fn handshake_fail_wrong_signature() {
    let (a, b) = MockUdp::pair_lossless();
    let mut client = MockHandshakeTransport::new(Arc::new(a));
    let mut server = MockHandshakeTransport::new(Arc::new(b));

    let client_id = DeviceIdentity {
        cid: uuid::Uuid::new_v4(),
        manufacturer: "ALNP".into(),
        model: "Client".into(),
        firmware_rev: "1.0".into(),
    };
    let server_id = DeviceIdentity {
        cid: uuid::Uuid::new_v4(),
        manufacturer: "ALNP".into(),
        model: "Server".into(),
        firmware_rev: "1.0".into(),
    };

    let caps = CapabilitySet {
        supports_encryption: true,
        supports_redundancy: false,
        max_universes: Some(8),
        vendor_data: None,
    };
    let proto = ProtocolVersion::alnp_v1();

    // Authenticator mismatch: controller uses static secret, server expects ed25519 signature.
    let controller = ClientHandshake {
        identity: client_id,
        capabilities: caps.clone(),
        protocol_version: proto.clone(),
        authenticator: StaticKeyAuthenticator::default(),
        key_exchange: X25519KeyExchange::new(),
        context: HandshakeContext::default(),
    };

    let signing = ed25519_dalek::SigningKey::from_bytes(&[9u8; 32]);
    let creds = NodeCredentials {
        signing: signing.clone(),
        verifying: signing.verifying_key(),
    };
    let server_auth = Ed25519Authenticator::new(creds);

    let server_driver = ServerHandshake {
        identity: server_id,
        capabilities: caps,
        protocol_version: proto,
        authenticator: server_auth,
        key_exchange: X25519KeyExchange::new(),
        context: HandshakeContext::default(),
    };

    let c = tokio::spawn(async move { controller.run(&mut client).await });
    let s = tokio::spawn(async move { server_driver.run(&mut server).await });

    let controller_res = c.await.unwrap();
    let server_res = s.await;
    assert!(controller_res.is_err() || server_res.is_err());
}
