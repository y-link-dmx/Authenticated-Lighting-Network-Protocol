#![allow(clippy::unwrap_used)]
use super::support::MockUdp;
use std::sync::Arc;
use super::common::MockHandshakeTransport;
use alnp::handshake::transport::ReliableControlChannel;
use alnp::handshake::{HandshakeMessage, HandshakeTransport};
use alnp::messages::{Acknowledge, ControlEnvelope, ControlHeader, ControlPayload, IdentifyResponse, Keepalive};

#[tokio::test]
async fn control_keepalive_reset() {
    let (c_udp, s_udp) = MockUdp::pair_lossless();
    let mut client = ReliableControlChannel::new(MockHandshakeTransport::new(Arc::new(c_udp)));

    tokio::spawn(async move {
        let mut transport = MockHandshakeTransport::new(Arc::new(s_udp));
        // send keepalive to reset attempts
        let _ = transport
            .send(HandshakeMessage::Keepalive(Keepalive { session_id: None, tick_ms: 100 }))
            .await;
        while let Ok(msg) = transport.recv().await {
            if let HandshakeMessage::Control(env) = msg {
                let ack = Acknowledge { header: env.header.clone(), ok: true, detail: None, signature: vec![] };
                let _ = transport.send(HandshakeMessage::Ack(ack)).await;
                break;
            }
        }
    });

    let env = ControlEnvelope {
        header: ControlHeader { seq: 0, nonce: vec![], timestamp_ms: 0 },
        payload: ControlPayload::IdentifyResponse(IdentifyResponse { acknowledged: true, detail: None }),
        signature: vec![],
    };

    let ack = client.send_reliable(env).await.expect("keepalive should allow retry");
    assert!(ack.ok);
}
