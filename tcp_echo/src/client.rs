use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:8080";
    let mut stream = TcpStream::connect(addr).await?;

    let msg = "Hello Tokio Socket".to_string();
    let _ = stream.write(msg.as_bytes()).await?;

    let mut buf = Vec::with_capacity(4096);
    stream.read_buf(&mut buf).await?;

    let received = String::from_utf8(buf).unwrap();
    println!("{}", received);
    Ok(())
}
