use crate::types::KvsResult;

/// Helper struct for KvsResponse
pub(super) struct KvsResponse(String);

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
    pub(super) fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::{errors::KvsError, kvs::response::KvsResponse, types::KvsResult};

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
