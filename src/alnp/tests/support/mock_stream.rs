use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;

use alnp::stream::SacnStreamAdapter;

#[derive(Clone, Default)]
pub struct MockStreamAdapter {
    pub sent: Arc<Mutex<HashMap<u16, Vec<Vec<u8>>>>>,
}

impl SacnStreamAdapter for MockStreamAdapter {
    fn send_universe(&self, universe: u16, payload: &[u8]) -> Result<(), String> {
        self.sent
            .lock()
            .entry(universe)
            .or_default()
            .push(payload.to_vec());
        Ok(())
    }

    fn subscribe_universe(&self, _universe: u16) -> Result<(), String> {
        Ok(())
    }
}
