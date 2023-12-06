use self::tree_element::{TreeElement, Value};

mod branch;
mod convert;
mod node;
mod tree_element;

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
