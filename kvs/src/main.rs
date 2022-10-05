use std::io;

mod errors;
mod expansion;
mod handlers;
mod kvs;
mod kvs_stream;
mod query;
mod types;

#[tokio::main]
async fn main() -> io::Result<()> {
    kvs::Kvs::new().addr("0.0.0.0:8080").start().await
}
