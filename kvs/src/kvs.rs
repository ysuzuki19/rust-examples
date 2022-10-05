use std::{io, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

use crate::{handlers, kvs_stream::KvsStream, query::Query, types::Store};

async fn process(mut kvs_stream: KvsStream, store: Arc<RwLock<Store>>) {
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
                let res = match Query::from_str(input.as_str()) {
                    Ok(query) => {
                        let store = store.clone();
                        match query {
                            Query::Get(args) => handlers::get(store, args).await,
                            Query::Set(args) => handlers::set(store, args).await,
                        }
                    }
                    Err(msg) => Err(msg),
                };
                kvs_stream.write_result(res).await;
            }
        }
    }
}

pub struct Kvs<'a> {
    store: Arc<RwLock<Store>>,
    addr: &'a str,
}

impl<'a> Kvs<'a> {
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

    pub async fn start(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.addr)
            .await
            .expect(format!("failed to listen {}", self.addr).as_str());
        loop {
            let (stream, addr) = listener.accept().await?;
            let kvs_stream = KvsStream::new(stream);
            let store = self.store.clone();
            tokio::spawn(async move { process(kvs_stream, store).await });
            println!("socket connected from {}", addr);
        }
    }
}
