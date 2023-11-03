#[derive(Debug, PartialEq)]
pub struct GetArgs<'a> {
    key: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct SetArgs<'a> {
    key: &'a str,
    val: &'a str,
}

impl<'a> GetArgs<'a> {
    const SIZE: usize = 1;

    pub fn new(strs: [&'a str; GetArgs::SIZE]) -> Self {
        Self { key: strs[0] }
    }

    pub fn key(&self) -> &str {
        self.key
    }
}

impl<'a> SetArgs<'a> {
    const SIZE: usize = 2;

    pub fn new(strs: [&'a str; SetArgs::SIZE]) -> Self {
        Self {
            key: strs[0],
            val: strs[1],
        }
    }

    pub fn key(&self) -> &str {
        self.key
    }

    pub fn val(&self) -> &str {
        self.val
    }
}
