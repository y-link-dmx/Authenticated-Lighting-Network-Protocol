#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, UdpLike};
use std::sync::Arc;
use super::common::MockHandshakeTransport;
use alnp::handshake::HandshakeTransport;

#[tokio::test]
async fn control_invalid_json() {
    let (c_udp, s_udp) = MockUdp::pair_lossless();
    let mut client = MockHandshakeTransport::new(Arc::new(c_udp));
    // Inject invalid bytes from server side.
    tokio::spawn(async move {
        let udp = Arc::new(s_udp);
        udp.send(vec![0xff, 0xff]).await;
    });

    let res = client.recv().await;
    assert!(res.is_err(), "invalid JSON should error");
}
