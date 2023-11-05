mod args;
mod method;
mod str_ssv_array;

use std::str::FromStr;

pub use self::args::*;
use self::{method::Method, str_ssv_array::SsvArray};
use crate::error::{KvsError, KvsResult};

#[derive(Debug, PartialEq)]
pub enum Query<'a> {
    Get(GetArgs<'a>),
    Set(SetArgs<'a>),
}

impl<'a> TryFrom<&'a str> for Query<'a> {
    type Error = KvsError;

    fn try_from(s: &'a str) -> Result<Query<'a>, Self::Error> {
        let (m, s) = parse_method(s)?;
        match m {
            Method::Get => {
                let args = GetArgs::new(s.ssv_array()?);
                Ok(Query::Get(args))
            }
            Method::Set => {
                let args = SetArgs::new(s.ssv_array()?);
                println!("{}", s);
                Ok(Query::Set(args))
            }
        }
    }
}

/// query parser for plain message
/// must to be formatted as `<method> <key> <...args>`
fn parse_method(query: &str) -> KvsResult<(Method, &str)> {
    match query.split_once(' ') {
        Some((key, subquery)) => {
            let method = Method::from_str(key)?;
            Ok((method, subquery))
        }
        None => Err(KvsError::InvalidQueryFormat),
    }
}

#[cfg(test)]
mod tests {
    use crate::query::{
        args::{GetArgs, SetArgs},
        Query,
    };

    #[test]
    fn get_query() {
        let input = "GET t";
        let query = Query::try_from(input);
        assert!(query.is_ok());
        assert_eq!(query.unwrap(), (Query::Get(GetArgs::new(["t"]))));
    }

    #[test]
    fn set_query() {
        let query = "SET t 1";
        let parsed = Query::try_from(query);
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), (Query::Set(SetArgs::new(["t", "1"]))));
    }
}
