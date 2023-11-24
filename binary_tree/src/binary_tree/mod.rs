mod convert;
mod node;

pub type Value = i32;

#[derive(Default)]
pub struct BinaryTree {
    root: node::Node,
}

impl BinaryTree {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, value: Value) {
        self.root.insert(value);
    }

    pub fn contains(&self, value: Value) -> bool {
        self.root.contains(value)
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }
}
