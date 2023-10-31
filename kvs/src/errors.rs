use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("stream error {0:?}")]
    StreamError(#[from] io::Error),

    #[error("stream disconnected")]
    StreamDisconnected,

    #[error("key not found: {0:?}")]
    KeyNotFound(String),

    #[error("Invalid Method Name: {0:?}")]
    InvalidMethodName(String),

    #[error("must to have space: <method> <key> <...args>")]
    InvalidQueryFormat,

    #[error("method must to have {0:?} payloads")]
    InvalidPayloadSize(usize),
}
