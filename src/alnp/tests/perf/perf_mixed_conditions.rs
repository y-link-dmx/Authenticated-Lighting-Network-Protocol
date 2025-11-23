#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockUdpConfig, MockStreamAdapter, *};
use std::sync::Arc;
use super::common::make_sessions;
use crate::assert_frame_delivery_ratio;

#[tokio::test]
#[ignore]
async fn perf_mixed_conditions() {
    let cfg = MockUdpConfig { loss_pct: 0.35, jitter_ms: 15, reorder_prob: 0.2, corrupt: false };
    let (a, b) = MockUdp::pair_ext(cfg);
    let (controller, _) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller, adapter.clone());

    let frames = 500;
    for i in 0..frames {
        let _ = stream.send(1, &[i as u8; 3]);
    }

    let delivered = adapter.sent.lock().get(&1).map(|v| v.len()).unwrap_or(0);
    assert_frame_delivery_ratio!(delivered, frames, 0.6);
}
