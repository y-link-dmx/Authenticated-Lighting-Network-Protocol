#![allow(clippy::unwrap_used)]
use super::common::*;
use super::support::{MockUdp, *};
use alnp::session::state::SessionState;
use std::sync::Arc;
use crate::assert_session_state;

#[tokio::test]
async fn handshake_ok() {
    let (a, b) = MockUdp::pair_lossless();
    let (controller, node) = make_sessions(Arc::new(a), Arc::new(b)).await;
    assert_session_state!(controller, SessionState::Ready { .. } | SessionState::Streaming { .. });
    assert_session_state!(node, SessionState::Ready { .. } | SessionState::Streaming { .. });
}
