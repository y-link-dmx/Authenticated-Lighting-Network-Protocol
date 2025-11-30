use alpine::stream::FrameTransport;

/// Placeholder QUIC transport for future SDK work.
#[derive(Debug)]
pub struct QuicFrameTransport;

impl FrameTransport for QuicFrameTransport {
    fn send_frame(&self, _bytes: &[u8]) -> Result<(), String> {
        Err("QUIC transport is not implemented yet".to_string())
    }
}
