use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{errors::KvsError, types::KvsResult};

/// TcpStream Wrapper for Kvs
pub struct KvsStream {
    stream: TcpStream,
}

impl KvsStream {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn read(&mut self) -> KvsResult<String> {
        let mut buf = Vec::with_capacity(4096);
        match self.stream.read_buf(&mut buf).await {
            Err(e) => Err(KvsError::StreamError(e)),
            Ok(0) => Err(KvsError::StreamDisconnected),
            Ok(_) => Ok(String::from_utf8(buf)
                .expect("failed to converting buffer to string")
                .trim()
                .into()),
        }
    }

    pub async fn write_result(&mut self, res: KvsResult<String>) {
        let msg = make_response(res);
        let buf = msg.as_bytes();
        let _ = self.stream.write(buf).await;
    }
}

/// Kvs Response for TcpStream
/// If command succeed, client get `Ok <res|msg>`
/// If command failed, client get `Er <msg>`
fn make_response(res: KvsResult<String>) -> String {
    match res {
        Ok(msg) => format!("Ok {msg}"),
        Err(msg) => format!("Er {msg}"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{errors::KvsError, kvs_stream::make_response, types::KvsResult};

    #[test]
    fn ok_message() {
        let res: KvsResult<String> = Ok("msg".to_owned());
        let msg = make_response(res);
        assert_eq!(msg, "Ok msg");
    }

    #[test]
    fn er_message() {
        let res: KvsResult<String> = Err(KvsError::InvalidQueryFormat);
        let msg = make_response(res);
        assert_eq!(msg, "Er must to have space: <method> <key> <...args>");
    }
}
