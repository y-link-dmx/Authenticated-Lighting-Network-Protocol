#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockUdpConfig, *};
use super::common::MockHandshakeTransport;
use std::sync::Arc;
use alnp::handshake::transport::ReliableControlChannel;
use alnp::handshake::{HandshakeMessage, HandshakeTransport};
use alnp::messages::{Acknowledge, ControlEnvelope, ControlHeader, ControlPayload, IdentifyResponse};
use crate::assert_latency;

async fn round_trip(loss: f32) -> u128 {
    let cfg = MockUdpConfig { loss_pct: loss, jitter_ms: 2, ..Default::default() };
    let (c_udp, s_udp) = MockUdp::pair_ext(cfg);
    let mut client = ReliableControlChannel::new(MockHandshakeTransport::new(Arc::new(c_udp)));
    tokio::spawn(async move {
        let mut server = MockHandshakeTransport::new(Arc::new(s_udp));
        while let Ok(msg) = server.recv().await {
            if let HandshakeMessage::Control(env) = msg {
                let ack = Acknowledge { header: env.header.clone(), ok: true, detail: None, signature: vec![] };
                let _ = server.send(HandshakeMessage::Ack(ack)).await;
            }
        }
    });

    let env = ControlEnvelope {
        header: ControlHeader { seq: 0, nonce: vec![], timestamp_ms: 0 },
        payload: ControlPayload::IdentifyResponse(IdentifyResponse { acknowledged: true, detail: None }),
        signature: vec![],
    };
    let start = std::time::Instant::now();
    let _ = client.send_reliable(env).await.unwrap();
    start.elapsed().as_millis()
}

#[tokio::test]
#[ignore]
async fn perf_control_latency() {
    assert_latency!(round_trip(0.0).await, 20);
    assert_latency!(round_trip(0.1).await, 50);
    assert_latency!(round_trip(0.3).await, 100);
}
