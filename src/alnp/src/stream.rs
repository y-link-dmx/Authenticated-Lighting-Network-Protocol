use thiserror::Error;

use crate::session::AlnpSession;

/// Minimal adapter that wraps the existing sACN transport layer.
pub trait SacnStreamAdapter: Send + Sync {
    fn send_universe(&self, universe: u16, payload: &[u8]) -> Result<(), String>;
    fn subscribe_universe(&self, universe: u16) -> Result<(), String>;
}

#[derive(Debug)]
pub struct AlnpStream<T: SacnStreamAdapter> {
    session: AlnpSession,
    sacn: T,
}

#[derive(Debug, Error)]
pub enum StreamError {
    #[error("sender not authenticated")]
    NotAuthenticated,
    #[error("sACN transport error: {0}")]
    Transport(String),
}

impl<T: SacnStreamAdapter> AlnpStream<T> {
    pub fn new(session: AlnpSession, sacn: T) -> Self {
        Self { session, sacn }
    }

    pub fn send(&self, universe: u16, payload: &[u8]) -> Result<(), StreamError> {
        self.session
            .ensure_established()
            .map_err(|_| StreamError::NotAuthenticated)?;
        self.sacn
            .send_universe(universe, payload)
            .map_err(StreamError::Transport)
    }

    pub fn subscribe(&self, universe: u16) -> Result<(), StreamError> {
        self.session
            .ensure_established()
            .map_err(|_| StreamError::NotAuthenticated)?;
        self.sacn
            .subscribe_universe(universe)
            .map_err(StreamError::Transport)
    }
}
