#[path = "support/mod.rs"]
mod support;
#[path = "e2e/common.rs"]
mod common;
#[path = "e2e/handshake_ok.rs"]
mod handshake_ok;
#[path = "e2e/handshake_fail_wrong_signature.rs"]
mod handshake_fail_wrong_signature;
#[path = "e2e/handshake_fail_replay.rs"]
mod handshake_fail_replay;
#[path = "e2e/handshake_timeout.rs"]
mod handshake_timeout;
#[path = "e2e/control_loss_recovery.rs"]
mod control_loss_recovery;
#[path = "e2e/control_reorder.rs"]
mod control_reorder;
#[path = "e2e/control_retransmit_backoff.rs"]
mod control_retransmit_backoff;
#[path = "e2e/control_keepalive_reset.rs"]
mod control_keepalive_reset;
#[path = "e2e/control_invalid_json.rs"]
mod control_invalid_json;
#[path = "e2e/streaming_basic.rs"]
mod streaming_basic;
#[path = "e2e/streaming_multiverse.rs"]
mod streaming_multiverse;
#[path = "e2e/streaming_sequence_rollover.rs"]
mod streaming_sequence_rollover;
#[path = "e2e/streaming_jitter_modes.rs"]
mod streaming_jitter_modes;
#[path = "e2e/streaming_toggle_modes.rs"]
mod streaming_toggle_modes;
#[path = "e2e/streaming_fail_closed.rs"]
mod streaming_fail_closed;
#[path = "e2e/failure_paths.rs"]
mod failure_paths;
