#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockStreamAdapter};
use std::sync::Arc;
use super::common::make_sessions;

#[tokio::test]
async fn streaming_sequence_rollover() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, _) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller, adapter.clone());

    for _ in 0..260 {
        stream.send(1, &[1, 2, 3]).unwrap();
    }
    let sent = adapter.sent.lock();
    assert!(sent.get(&1).map(|v| v.len() >= 260).unwrap_or(false));
}
