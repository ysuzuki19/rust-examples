use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{error::KvsResult, query::SetArgs, types::Store};

pub async fn set(store: Arc<RwLock<Store>>, args: SetArgs<'_>) -> KvsResult<String> {
    store
        .write()
        .await
        .insert(args.key().into(), args.val().into());
    Ok("Succeed to insert".to_owned())
}
