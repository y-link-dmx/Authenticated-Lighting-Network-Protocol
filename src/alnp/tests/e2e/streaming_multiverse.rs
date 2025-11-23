#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockStreamAdapter};
use std::sync::Arc;
use super::common::make_sessions;

#[tokio::test]
async fn streaming_multiverse() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, _) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller, adapter.clone());

    for u in 1..=8 {
        stream.send(u, &[u as u8; 4]).unwrap();
    }
    let sent = adapter.sent.lock();
    assert_eq!(sent.len(), 8);
}
