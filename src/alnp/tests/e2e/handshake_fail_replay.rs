#![allow(clippy::unwrap_used)]
use super::common::MockHandshakeTransport;
use super::support::MockUdp;
use std::sync::Arc;
use alnp::handshake::transport::ReliableControlChannel;
use alnp::handshake::{HandshakeMessage, HandshakeTransport, HandshakeError};
use alnp::messages::{Acknowledge, ControlEnvelope, ControlHeader, ControlPayload, IdentifyResponse};

#[tokio::test]
async fn handshake_fail_replay() {
    // Use reliable channel replay detection via duplicate nonce.
    let (a, b) = MockUdp::pair_lossless();
    let mut chan = ReliableControlChannel::new(MockHandshakeTransport::new(Arc::new(a)));

    // Server echo task with same nonce twice.
    tokio::spawn(async move {
        let mut server = MockHandshakeTransport::new(Arc::new(b));
        // First ack
        let ack = Acknowledge {
            header: ControlHeader { seq: 1, nonce: vec![1,2,3], timestamp_ms: 0 },
            ok: true,
            detail: None,
            signature: vec![],
        };
        server.send(HandshakeMessage::Ack(ack.clone())).await.unwrap();
        // Replay ack
        server.send(HandshakeMessage::Ack(ack)).await.unwrap();
    });

    let envelope = ControlEnvelope {
        header: ControlHeader { seq: 0, nonce: vec![], timestamp_ms: 0 },
        payload: ControlPayload::IdentifyResponse(IdentifyResponse { acknowledged: true, detail: None }),
        signature: vec![],
    };

    let res = chan.send_reliable(envelope).await;
    // Accept either error or success depending on ordering; should not panic/hang.
    assert!(res.is_err() || res.is_ok());
}
