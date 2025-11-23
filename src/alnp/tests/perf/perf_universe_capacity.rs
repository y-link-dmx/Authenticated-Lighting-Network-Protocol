#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, MockStreamAdapter, *};
use std::sync::Arc;
use super::common::make_sessions;
use crate::assert_fps_min;

#[path = "../e2e/common.rs"]
mod common;

#[tokio::test]
#[ignore]
async fn perf_universe_capacity() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, _) = make_sessions(Arc::new(a), Arc::new(b)).await;
    let adapter = MockStreamAdapter::default();
    let stream = alnp::stream::AlnpStream::new(controller, adapter.clone());

    let universes = [8u16, 16, 24, 32];
    for &u in &universes {
        let start = std::time::Instant::now();
        let frames = 200;
        for _ in 0..frames {
            for uni in 1..=u {
                let _ = stream.send(uni, &[1; 4]);
            }
        }
        let elapsed = start.elapsed().as_secs_f64();
        let fps = (frames as f64) / elapsed;
        let min = if u >= 32 { 30.0 } else { 40.0 };
        assert_fps_min!(fps, min);
    }
}
