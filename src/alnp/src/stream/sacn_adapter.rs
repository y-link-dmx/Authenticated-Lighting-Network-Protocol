//! FFI bridge to the existing C sACN sender/receiver.

use super::SacnStreamAdapter;

type SacnSourceHandle = i32;
type SacnReceiverHandle = i32;

extern "C" {
    fn sacn_source_update_levels_and_force_sync(
        handle: SacnSourceHandle,
        universe: u16,
        new_levels: *const u8,
        new_levels_size: usize,
    );

    fn sacn_receiver_change_universe(handle: SacnReceiverHandle, new_universe_id: u16) -> i32;
}

/// Safe wrapper around the C sACN functions required to send or listen to universes.
pub struct CSacnAdapter {
    pub source: SacnSourceHandle,
    pub receiver: SacnReceiverHandle,
}

impl SacnStreamAdapter for CSacnAdapter {
    fn send_universe(&self, universe: u16, payload: &[u8]) -> Result<(), String> {
        unsafe {
            sacn_source_update_levels_and_force_sync(
                self.source,
                universe,
                payload.as_ptr(),
                payload.len(),
            );
        }
        Ok(())
    }

    fn subscribe_universe(&self, universe: u16) -> Result<(), String> {
        let res = unsafe { sacn_receiver_change_universe(self.receiver, universe) };
        if res == 0 {
            Ok(())
        } else {
            Err(format!("sACN receiver change_universe error {}", res))
        }
    }
}
