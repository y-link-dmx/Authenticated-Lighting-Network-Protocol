#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockUdpConfig};
use super::common::MockHandshakeTransport;
use std::sync::Arc;
use alnp::handshake::transport::ReliableControlChannel;
use alnp::handshake::{HandshakeMessage, HandshakeTransport};
use alnp::messages::{Acknowledge, ControlEnvelope, ControlHeader, ControlPayload, IdentifyResponse};

async fn run_server(mut transport: MockHandshakeTransport) {
    while let Ok(msg) = transport.recv().await {
        if let HandshakeMessage::Control(env) = msg {
            let ack = Acknowledge {
                header: env.header.clone(),
                ok: true,
                detail: None,
                signature: vec![],
            };
            let _ = transport.send(HandshakeMessage::Ack(ack)).await;
        }
    }
}

#[tokio::test]
async fn control_loss_recovery() {
    let cfg = MockUdpConfig { loss_pct: 0.25, jitter_ms: 2, ..Default::default() };
    let (c_udp, s_udp) = MockUdp::pair_ext(cfg);
    let mut client = ReliableControlChannel::new(MockHandshakeTransport::new(Arc::new(c_udp)));

    tokio::spawn(run_server(MockHandshakeTransport::new(Arc::new(s_udp))));

    let env = ControlEnvelope {
        header: ControlHeader { seq: 0, nonce: vec![], timestamp_ms: 0 },
        payload: ControlPayload::IdentifyResponse(IdentifyResponse { acknowledged: true, detail: None }),
        signature: vec![],
    };

    let ack = client.send_reliable(env).await.expect("should recover");
    assert!(ack.ok);
}
