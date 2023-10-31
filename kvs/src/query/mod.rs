pub mod args;
mod method;

use std::str::FromStr;

use self::{args::*, method::Method};
use crate::{errors::KvsError, expansion::str::SsvArray, types::KvsResult};

#[derive(Debug, PartialEq)]
pub enum Query<'a> {
    Get(GetArgs<'a>),
    Set(SetArgs<'a>),
}

impl<'a> Query<'a> {
    pub fn from_str(s: &'a str) -> KvsResult<Query<'a>> {
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
        let query = Query::from_str(input);
        assert!(query.is_ok());
        assert_eq!(query.unwrap(), (Query::Get(GetArgs { key: "t" })));
    }

    #[test]
    fn set_query() {
        let query = "SET t 1";
        let parsed = Query::from_str(query);
        assert!(parsed.is_ok());
        assert_eq!(
            parsed.unwrap(),
            (Query::Set(SetArgs { key: "t", val: "1" }))
        );
    }
}
