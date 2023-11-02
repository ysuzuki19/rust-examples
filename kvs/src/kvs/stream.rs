use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{errors::KvsError, types::KvsResult};

use super::response::KvsResponse;

/// TcpStream Wrapper for Kvs
pub struct KvsStream(pub(super) TcpStream);

impl KvsStream {
    pub async fn read(&mut self) -> KvsResult<String> {
        let mut buf = Vec::with_capacity(4096);
        match self.0.read_buf(&mut buf).await {
            Err(e) => Err(KvsError::StreamError(e)),
            Ok(0) => Err(KvsError::StreamDisconnected),
            Ok(_) => Ok(String::from_utf8(buf)?.trim().into()),
        }
    }

    /// Write result message to TcpStream
    pub async fn write_result(&mut self, res: KvsResult<String>) {
        let _ = self.0.write(&KvsResponse::from(res).into_bytes()).await;
    }
}
