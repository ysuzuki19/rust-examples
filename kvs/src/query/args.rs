#[derive(Debug, PartialEq)]
pub struct GetArgs<'a> {
    pub key: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct SetArgs<'a> {
    pub key: &'a str,
    pub val: &'a str,
}

impl<'a> GetArgs<'a> {
    pub const SIZE: usize = 1;

    pub fn new(strs: [&'a str; GetArgs::SIZE]) -> Self {
        Self { key: strs[0] }
    }
}

impl<'a> SetArgs<'a> {
    pub const SIZE: usize = 2;

    pub fn new(strs: [&'a str; SetArgs::SIZE]) -> Self {
        Self {
            key: strs[0],
            val: strs[1],
        }
    }
}
