//! Authenticated Lighting Network Protocol (ALNP) scaffolding.
//!
//! This crate layers an authenticated control plane over the existing sACN
//! transport while keeping packet formats untouched. The streaming path is
//! guarded by a handshake derived from ESTA E1.33 patterns.

pub mod crypto;
pub mod handshake;
pub mod messages;
pub mod session;
pub mod stream;

pub use session::{AlnpRole, AlnpSession};
