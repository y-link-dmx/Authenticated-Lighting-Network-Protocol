#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockStreamAdapter};
use std::sync::Arc;
use super::common::make_sessions;
use alnp::session::JitterStrategy;

#[tokio::test]
async fn streaming_jitter_modes() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, _) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller.clone(), adapter.clone());

    // HoldLast
    controller.set_jitter_strategy(JitterStrategy::HoldLast);
    stream.send(1, &[9, 9, 9]).unwrap();
    stream.send(1, &[]).unwrap();
    let last = adapter.sent.lock().get(&1).unwrap().last().unwrap().clone();
    assert_eq!(last, vec![9, 9, 9]);

    // Drop
    controller.set_jitter_strategy(JitterStrategy::Drop);
    stream.send(1, &[]).unwrap();
    // Lerp
    controller.set_jitter_strategy(JitterStrategy::Lerp);
    stream.send(1, &[10, 10, 10]).unwrap();
}
