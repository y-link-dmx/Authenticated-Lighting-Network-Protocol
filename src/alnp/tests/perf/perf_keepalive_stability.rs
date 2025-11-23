#![allow(clippy::unwrap_used)]
use super::support::{MockUdp, *};
use super::common::{make_sessions, start_keepalive};
use std::sync::Arc;
use alnp::session::state::SessionState;
use crate::assert_session_state;

#[tokio::test]
#[ignore]
async fn perf_keepalive_stability() {
    let (a, b) = MockUdp::pair_with(0.1, 5);
    let (controller, node) = make_sessions(Arc::new(a), Arc::new(b)).await;

    // Start keepalives on loopback to keep state fresh.
    let loopback = alnp::session::LoopbackTransport::new();
    start_keepalive(loopback, None, 50).await;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    assert_session_state!(controller, SessionState::Ready { .. } | SessionState::Streaming { .. });
    assert_session_state!(node, SessionState::Ready { .. } | SessionState::Streaming { .. });
}
