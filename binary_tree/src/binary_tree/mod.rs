use self::{
    common::{TreeElement, Value},
    iter::InOrderIter,
};

mod branch;
mod common;
mod convert;
mod iter;
mod node;

#[derive(Default, Clone)]
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

impl BinaryTree {
    pub fn iter(&self) -> InOrderIter {
        InOrderIter::new(&self.root)
    }
}
