use std::collections::HashMap;

use crate::errors::KvsError;

pub type Store = HashMap<String, String>;
pub type KvsResult<T> = Result<T, KvsError>;
