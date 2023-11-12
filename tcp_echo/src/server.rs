use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;

    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {
                let mut buf = Vec::with_capacity(4096);
                socket.read_buf(&mut buf).await?;

                let msg = String::from_utf8(buf).expect("failed to convert str");
                println!("{msg}");

                let _ = socket.write(msg.as_bytes()).await?;
            }
            Err(err) => {
                println!("{err:?}");
            }
        };
    }
}
