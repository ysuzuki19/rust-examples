use std::str::FromStr;

use crate::errors::KvsError;

/// Method for Kvs
#[derive(Debug, PartialEq)]
pub enum Method {
    Set, // mutable
    Get, // immutable
}

impl FromStr for Method {
    type Err = KvsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SET" => Ok(Method::Set),
            "GET" => Ok(Method::Get),
            _ => Err(KvsError::InvalidMethodName(s.into())),
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Self::Get => "GET".to_owned(),
            Self::Set => "SET".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod from_str {
        use std::str::FromStr;

        use crate::query::method::Method;

        fn valid_method(s: &str) -> Method {
            let method_res = Method::from_str(s);
            assert!(method_res.is_ok());
            method_res.unwrap()
        }

        #[test]
        fn valid_method_set() {
            assert_eq!(valid_method("SET"), Method::Set);
        }

        #[test]
        fn valid_method_get() {
            assert_eq!(valid_method("GET"), Method::Get);
        }

        #[test]
        fn invalid_method() {
            assert!(Method::from_str("HOGE").is_err());
        }
    }

    mod to_str {
        use crate::query::method::Method;

        #[test]
        fn method_get_to_string() {
            assert_eq!(Method::Get.to_string(), "GET");
        }

        #[test]
        fn method_set_to_string() {
            assert_eq!(Method::Set.to_string(), "SET");
        }
    }
}
