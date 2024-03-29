use super::{common::Value, node};

#[derive(Default)]
pub struct InOrderIter<'a> {
    stack: Vec<&'a node::Node>,
}

impl<'a> InOrderIter<'a> {
    pub fn new(root: &'a node::Node) -> Self {
        let mut iter = Self::default();
        iter.push_recursively(root);
        iter
    }

    fn push_recursively(&mut self, node: &'a node::Node) {
        self.stack.push(node);
        if let node::Node::Branch(branch) = node {
            self.push_recursively(branch.left());
        }
    }
}

impl<'a> Iterator for InOrderIter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        match node {
            node::Node::Branch(branch) => {
                self.push_recursively(branch.right());
                Some(branch.value())
            }
            node::Node::Empty => self.next(),
        }
    }
}
