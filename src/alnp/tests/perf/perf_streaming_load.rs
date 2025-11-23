#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockStreamAdapter};
use std::sync::Arc;
use super::common::make_sessions;

#[tokio::test]
#[ignore]
async fn perf_streaming_load() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, _) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller, adapter.clone());

    let frames = 10_000;
    for i in 0..frames {
        stream.send(1, &[i as u8; 4]).unwrap();
    }

    let sent = adapter.sent.lock();
    let delivered = sent.get(&1).map(|v| v.len()).unwrap_or(0);
    assert!(delivered >= frames as usize);
}
