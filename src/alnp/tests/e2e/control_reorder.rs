#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockUdpConfig};
use super::common::MockHandshakeTransport;
use std::sync::Arc;
use alnp::handshake::transport::ReliableControlChannel;
use alnp::handshake::{HandshakeMessage, HandshakeTransport};
use alnp::messages::{Acknowledge, ControlEnvelope, ControlHeader, ControlPayload, IdentifyResponse};

async fn server(mut transport: MockHandshakeTransport) {
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
async fn control_reorder() {
    let cfg = MockUdpConfig {
        loss_pct: 0.0,
        jitter_ms: 1,
        reorder_prob: 0.2,
        corrupt: false,
    };

    let (c_udp, s_udp) = MockUdp::pair_ext(cfg);
    let mut client = ReliableControlChannel::new(
        MockHandshakeTransport::new(Arc::new(c_udp))
    );

    tokio::spawn(server(
        MockHandshakeTransport::new(Arc::new(s_udp))
    ));

    let env = ControlEnvelope {
        header: ControlHeader {
            seq: 0,
            nonce: vec![],
            timestamp_ms: 0,
        },
        payload: ControlPayload::IdentifyResponse(
            IdentifyResponse {
                acknowledged: true,
                detail: None,
            }
        ),
        signature: vec![],
    };

    let ack = tokio::time::timeout(
        std::time::Duration::from_millis(700),
        client.send_reliable(env)
    )
        .await
        .expect("no hang")
        .expect("reordered packets recovered");

    assert!(ack.ok);
}
