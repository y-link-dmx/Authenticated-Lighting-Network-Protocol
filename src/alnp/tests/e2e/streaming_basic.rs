#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockStreamAdapter};
use std::sync::Arc;
use super::common::make_sessions;

#[tokio::test]
async fn streaming_basic() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, node) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller, adapter.clone());
    stream.send(1, &[1, 2, 3, 4]).unwrap();
    let sent = adapter.sent.lock();
    assert!(sent.get(&1).map(|v| !v.is_empty()).unwrap_or(false));
    // ensure node ready too
    assert!(node.established().is_some());
}
