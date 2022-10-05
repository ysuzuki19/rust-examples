use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    query::args::SetArgs,
    types::{KvsResult, Store},
};

pub async fn set<'a>(store: Arc<RwLock<Store>>, args: SetArgs<'a>) -> KvsResult<String> {
    store.write().await.insert(args.key.into(), args.val.into());
    Ok("Succeed to insert".to_owned())
}
