use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{errors::KvsError, types::KvsResult};

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
        let buf = KvsResponse::from(res).into_bytes();
        let _ = self.0.write(&buf).await;
    }
}

/// Helper struct for KvsResponse
struct KvsResponse(String);

impl From<KvsResult<String>> for KvsResponse {
    /// Kvs Response for TcpStream
    /// If command succeed, client get `Ok <res|msg>`
    /// If command failed, client get `Er <msg>`
    fn from(res: KvsResult<String>) -> Self {
        match res {
            Ok(msg) => Self(format!("Ok {msg}")),
            Err(msg) => Self(format!("Er {msg}")),
        }
    }
}

impl KvsResponse {
    fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::{errors::KvsError, kvs_stream::KvsResponse, types::KvsResult};

    #[test]
    fn ok_message() {
        let res: KvsResult<String> = Ok("msg".to_owned());
        let buf = KvsResponse::from(res).into_bytes();
        assert_eq!(buf, "Ok msg".as_bytes());
    }

    #[test]
    fn er_message() {
        let res: KvsResult<String> = Err(KvsError::InvalidQueryFormat);
        let buf = KvsResponse::from(res).into_bytes();
        assert_eq!(
            buf,
            "Er must to have space: <method> <key> <...args>".as_bytes()
        );
    }
}
