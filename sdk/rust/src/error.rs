use std::fmt;

use alpine_protocol_rs::handshake::HandshakeError;
use alpine_protocol_rs::stream::StreamError;

/// Errors emitted by the SDK client.
#[derive(Debug)]
#[non_exhaustive]
pub enum AlpineSdkError {
    Io(String),
    Handshake(HandshakeError),
    Stream(StreamError),
}

impl fmt::Display for AlpineSdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlpineSdkError::Io(err) => write!(f, "io error: {}", err),
            AlpineSdkError::Handshake(err) => write!(f, "handshake error: {}", err),
            AlpineSdkError::Stream(err) => write!(f, "stream error: {}", err),
        }
    }
}

impl From<HandshakeError> for AlpineSdkError {
    fn from(err: HandshakeError) -> Self {
        AlpineSdkError::Handshake(err)
    }
}

impl From<StreamError> for AlpineSdkError {
    fn from(err: StreamError) -> Self {
        AlpineSdkError::Stream(err)
    }
}

impl From<std::io::Error> for AlpineSdkError {
    fn from(err: std::io::Error) -> Self {
        AlpineSdkError::Io(err.to_string())
    }
}
