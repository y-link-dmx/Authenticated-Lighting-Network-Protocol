#![allow(clippy::unwrap_used)]
use std::sync::Arc;
use super::support::{MockUdp, MockUdpConfig};
use super::common::MockHandshakeTransport;
use alnp::handshake::HandshakeParticipant;
use alnp::handshake::client::ClientHandshake;
use alnp::handshake::server::ServerHandshake;
use alnp::handshake::HandshakeContext;
use alnp::messages::{CapabilitySet, DeviceIdentity, ProtocolVersion};
use alnp::session::StaticKeyAuthenticator;
use alnp::crypto::X25519KeyExchange;

#[tokio::test]
async fn handshake_timeout() {
    // 100% loss: handshake should time out / error under timeout wrapper.
    let config = MockUdpConfig { loss_pct: 1.0, ..Default::default() };
    let (a, b) = MockUdp::pair_ext(config);
    let mut client = MockHandshakeTransport::new(Arc::new(a));
    let mut server = MockHandshakeTransport::new(Arc::new(b));

    let caps = CapabilitySet { supports_encryption: true, supports_redundancy: false, max_universes: Some(1), vendor_data: None };
    let proto = ProtocolVersion::alnp_v1();
    let client_driver = ClientHandshake {
        identity: DeviceIdentity { cid: uuid::Uuid::new_v4(), manufacturer: "A".into(), model: "c".into(), firmware_rev: "1".into() },
        capabilities: caps.clone(),
        protocol_version: proto.clone(),
        authenticator: StaticKeyAuthenticator::default(),
        key_exchange: X25519KeyExchange::new(),
        context: HandshakeContext::default(),
    };
    let server_driver = ServerHandshake {
        identity: DeviceIdentity { cid: uuid::Uuid::new_v4(), manufacturer: "B".into(), model: "s".into(), firmware_rev: "1".into() },
        capabilities: caps,
        protocol_version: proto,
        authenticator: StaticKeyAuthenticator::default(),
        key_exchange: X25519KeyExchange::new(),
        context: HandshakeContext::default(),
    };

    let c = tokio::spawn(async move { client_driver.run(&mut client).await });
    let s = tokio::spawn(async move { server_driver.run(&mut server).await });

    let controller_res = tokio::time::timeout(std::time::Duration::from_millis(200), c).await;
    assert!(controller_res.is_err() || controller_res.unwrap().is_err());
    let _ = s.abort();
}
