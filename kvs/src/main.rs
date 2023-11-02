mod errors;
mod expansion;
mod handlers;
mod kvs;
mod query;
mod types;

#[tokio::main]
async fn main() -> types::KvsResult<()> {
    kvs::Kvs::new().addr("0.0.0.0:8080").start().await
}
