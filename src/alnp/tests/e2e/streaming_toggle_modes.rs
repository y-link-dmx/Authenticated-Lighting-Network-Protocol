#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockStreamAdapter};
use std::sync::Arc;
use super::common::make_sessions;
use alnp::messages::OperatingMode;

#[tokio::test]
async fn streaming_toggle_modes() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, _) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller, adapter.clone());

    stream.set_mode(OperatingMode::Calibration);
    let err = stream.send(1, &[1, 2, 3]).unwrap_err();
    assert!(matches!(err, alnp::stream::StreamError::StreamingDisabled));

    stream.set_mode(OperatingMode::Normal);
    stream.send(1, &[4, 5, 6]).unwrap();
    assert!(adapter.sent.lock().get(&1).is_some());
}
