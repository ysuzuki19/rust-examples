use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    errors::KvsError,
    query::args::GetArgs,
    types::{KvsResult, Store},
};

pub async fn get(store: Arc<RwLock<Store>>, args: GetArgs<'_>) -> KvsResult<String> {
    match store.read().await.get(args.key()) {
        Some(val) => Ok(val.clone()),
        None => Err(KvsError::KeyNotFound(args.key().into())),
    }
}
