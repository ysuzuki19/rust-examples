use std::sync::Arc;

use tokio::{net::TcpListener, sync::RwLock};

use crate::{error::KvsResult, handlers, query::Query, types::Store};

use self::stream::KvsStream;

mod response;
mod stream;

async fn process(mut kvs_stream: KvsStream, store: Arc<RwLock<Store>>) -> KvsResult<()> {
    loop {
        match kvs_stream.read().await {
            Err(e) => {
                println!("Error occured: {}", e);
                break;
            }
            Ok(input) => {
                if !input.is_empty() {
                    println!("{}", input);
                }
                let res = match Query::try_from(input.as_str()) {
                    Ok(query) => match query {
                        Query::Get(args) => handlers::get(store.clone(), args).await,
                        Query::Set(args) => handlers::set(store.clone(), args).await,
                    },
                    Err(msg) => Err(msg),
                };
                kvs_stream.write_result(res).await?;
            }
        }
    }
    Ok(())
}

pub struct KvsServer<'a> {
    store: Arc<RwLock<Store>>,
    addr: &'a str,
}

impl<'a> KvsServer<'a> {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(Store::new())),
            addr: "0.0.0.0:8080",
        }
    }

    pub fn addr(mut self, addr: &'a str) -> Self {
        self.addr = addr;
        self
    }

    pub async fn start(&mut self) -> KvsResult<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        loop {
            let (stream, addr) = listener.accept().await?;
            let kvs_stream = KvsStream(stream);
            tokio::spawn({
                let store = self.store.clone();
                async move { process(kvs_stream, store).await }
            });
            println!("socket connected from {}", addr);
        }
    }
}
