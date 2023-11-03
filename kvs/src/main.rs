mod error;
mod handlers;
mod kvs;
mod query;
mod types;

#[tokio::main]
async fn main() -> error::KvsResult<()> {
    kvs::KvsServer::new().addr("0.0.0.0:8080").start().await
}
