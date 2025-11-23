#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockStreamAdapter};
use std::sync::Arc;
use super::common::make_sessions;

#[tokio::test]
async fn streaming_fail_closed() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, _) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller, adapter.clone());

    stream.fail_closed("test failure");
    let res = stream.send(1, &[1, 2, 3]);
    assert!(res.is_err());
}
