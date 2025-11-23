mod sacn_adapter;
pub use sacn_adapter::CSacnAdapter;

use std::collections::HashMap;

use thiserror::Error;

use crate::messages::OperatingMode;
use crate::session::{AlnpSession, JitterStrategy};

/// Minimal adapter that wraps the existing sACN transport layer.
pub trait SacnStreamAdapter: Send + Sync {
    fn send_universe(&self, universe: u16, payload: &[u8]) -> Result<(), String>;
    fn subscribe_universe(&self, universe: u16) -> Result<(), String>;
    fn unsubscribe_universe(&self, universe: u16) -> Result<(), String> {
        let _ = universe;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AlnpStream<T: SacnStreamAdapter> {
    session: AlnpSession,
    sacn: T,
    last_frames: parking_lot::Mutex<HashMap<u16, Vec<u8>>>,
    sequence_numbers: parking_lot::Mutex<HashMap<u16, u8>>,
    enabled_universes: parking_lot::Mutex<HashMap<u16, bool>>,
    current_mode: parking_lot::Mutex<OperatingMode>,
}

#[derive(Debug, Error)]
pub enum StreamError {
    #[error("sender not authenticated")]
    NotAuthenticated,
    #[error("sACN transport error: {0}")]
    Transport(String),
    #[error("streaming disabled")]
    StreamingDisabled,
}

impl<T: SacnStreamAdapter> AlnpStream<T> {
    pub fn new(session: AlnpSession, sacn: T) -> Self {
        Self {
            session,
            sacn,
            last_frames: parking_lot::Mutex::new(HashMap::new()),
            sequence_numbers: parking_lot::Mutex::new(HashMap::new()),
            enabled_universes: parking_lot::Mutex::new(HashMap::new()),
            current_mode: parking_lot::Mutex::new(OperatingMode::Normal),
        }
    }

    pub fn set_mode(&self, mode: OperatingMode) {
        *self.current_mode.lock() = mode.clone();
        let allow_stream = matches!(mode, OperatingMode::Normal);
        self.session.set_streaming_enabled(allow_stream);
    }

    pub fn enable_universe(&self, universe: u16) {
        self.enabled_universes.lock().insert(universe, true);
    }

    pub fn disable_universe(&self, universe: u16) {
        self.enabled_universes.lock().insert(universe, false);
        let _ = self.sacn.unsubscribe_universe(universe);
    }

    pub fn send(&self, universe: u16, payload: &[u8]) -> Result<(), StreamError> {
        let _session = self
            .session
            .ensure_streaming_ready()
            .map_err(|_| StreamError::NotAuthenticated)?;

        self.session.mark_streaming();

        if !self.session.streaming_enabled() {
            return Err(StreamError::StreamingDisabled);
        }

        if !self.enabled_universes.lock().get(&universe).cloned().unwrap_or(true) {
            return Err(StreamError::StreamingDisabled);
        }

        let seq = {
            let mut seqs = self.sequence_numbers.lock();
            let entry = seqs.entry(universe).or_insert(0);
            *entry = entry.wrapping_add(1);
            *entry
        };

        // Sequence rollover guard: if wraps to 0, ensure next send resets last frame to avoid jitter.
        if seq == 0 {
            self.last_frames.lock().remove(&universe);
        }

        let payload = self.apply_jitter(universe, payload);

        self.sacn
            .send_universe(universe, &payload)
            .map_err(StreamError::Transport)?;
        self.last_frames
            .lock()
            .insert(universe, payload.to_vec());
        Ok(())
    }

    pub fn subscribe(&self, universe: u16) -> Result<(), StreamError> {
        self.session
            .ensure_streaming_ready()
            .map_err(|_| StreamError::NotAuthenticated)?;
        self.sacn
            .subscribe_universe(universe)
            .map_err(StreamError::Transport)?;
        self.enable_universe(universe);
        Ok(())
    }

    pub fn fail_closed(&self, reason: &str) {
        self.session.fail(reason.to_string());
        let mut enabled = self.enabled_universes.lock();
        for (u, flag) in enabled.iter_mut() {
            if *flag {
                let _ = self.sacn.unsubscribe_universe(*u);
            }
            *flag = false;
        }
    }

    fn apply_jitter(&self, universe: u16, payload: &[u8]) -> Vec<u8> {
        match self.session.jitter_strategy() {
            JitterStrategy::HoldLast => {
                if payload.is_empty() {
                    if let Some(last) = self.last_frames.lock().get(&universe) {
                        return last.clone();
                    }
                }
                payload.to_vec()
            }
            JitterStrategy::Drop => {
                if payload.is_empty() {
                    Vec::new()
                } else {
                    payload.to_vec()
                }
            }
            JitterStrategy::Lerp => {
                let mut blended = payload.to_vec();
                if let Some(last) = self.last_frames.lock().get(&universe) {
                    let len = blended.len().min(last.len());
                    for i in 0..len {
                        blended[i] = ((last[i] as u16 + blended[i] as u16) / 2) as u8;
                    }
                }
                blended
            }
        }
    }
}
