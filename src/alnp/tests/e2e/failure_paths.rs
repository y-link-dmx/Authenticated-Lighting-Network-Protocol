#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, UdpLike};
use super::common::MockHandshakeTransport;
use std::sync::Arc;
use alnp::handshake::HandshakeTransport;

#[tokio::test]
async fn truncated_packet_rejected() {
    let (a, b) = MockUdp::pair_lossless();
    let mut client = MockHandshakeTransport::new(Arc::new(a));
    let b_arc = Arc::new(b);
    let sender = b_arc.clone();
    tokio::spawn(async move {
        sender.send(vec![0u8]).await;
    });
    let res = client.recv().await;
    assert!(res.is_err());
}

#[tokio::test]
async fn malicious_peer_close_channel() {
    let (a, _b) = MockUdp::pair_lossless();
    let mut client = MockHandshakeTransport::new(Arc::new(a));
    // No responder; channel will close -> transport error
    let res = tokio::time::timeout(std::time::Duration::from_millis(100), client.recv()).await;
    assert!(res.is_err() || res.unwrap().is_err());
}
