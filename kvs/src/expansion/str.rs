use crate::{errors::KvsError, types::KvsResult};

pub trait SsvArray {
    fn ssv_array<const N: usize>(&self) -> KvsResult<[&str; N]>;
}

impl SsvArray for str {
    fn ssv_array<const N: usize>(&self) -> KvsResult<[&str; N]> {
        let strs = self.split(' ').collect::<Vec<&str>>();
        match strs.try_into() {
            Ok(arr) => Ok(arr),
            Err(_) => Err(KvsError::InvalidPayloadSize(N)),
        }
    }
}

#[cfg(test)]
mod tests {
    mod parser {
        use crate::expansion::str::SsvArray;

        #[test]
        fn payloads_1() {
            let strs = "t".ssv_array::<1>();
            assert!(strs.is_ok());
            assert_eq!(strs.unwrap(), ["t"]);

            let strs = "t t".ssv_array::<1>();
            assert!(strs.is_err());

            let strs = "t t t".ssv_array::<1>();
            assert!(strs.is_err());
        }

        #[test]
        fn payloads_2() {
            let strs = "t".ssv_array::<2>();
            assert!(strs.is_err());

            let strs = "t t".ssv_array::<2>();
            assert!(strs.is_ok());
            assert_eq!(strs.unwrap(), ["t", "t"]);

            let strs = "t t t".ssv_array::<2>();
            assert!(strs.is_err());
        }
    }
}
