use self::{
    iter::InOrderIter,
    traits::{TreeElement, Value},
};

mod branch;
mod convert;
mod iter;
mod node;
mod traits;

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
        let mut iter = InOrderIter::default();
        iter.push_recursively(&self.root);
        iter
    }
}
