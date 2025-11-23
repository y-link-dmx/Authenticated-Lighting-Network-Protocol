#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockUdpConfig};
use super::common::MockHandshakeTransport;
use std::sync::Arc;
use alnp::handshake::transport::ReliableControlChannel;
use alnp::handshake::{HandshakeMessage, HandshakeTransport};
use alnp::messages::{Acknowledge, ControlEnvelope, ControlHeader, ControlPayload, IdentifyResponse};

#[tokio::test]
async fn control_retransmit_backoff() {
    let cfg = MockUdpConfig { loss_pct: 0.2, jitter_ms: 5, ..Default::default() };
    let (c_udp, s_udp) = MockUdp::pair_ext(cfg);
    let mut client = ReliableControlChannel::new(MockHandshakeTransport::new(Arc::new(c_udp)));

    tokio::spawn(async move {
        let mut transport = MockHandshakeTransport::new(Arc::new(s_udp));
        // Wait to simulate backoff need
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
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
    let ack = client.send_reliable(env).await.expect("backoff should succeed");
    assert!(ack.ok);
}
