use super::{node::Node, traits::TreeElement, Value};

#[derive(Clone)]
pub struct Branch {
    value: Value,
    size: usize,
    left: Node,
    right: Node,
}

impl Branch {
    pub fn new(value: Value) -> Branch {
        Branch {
            value,
            size: 1,
            left: Node::Empty,
            right: Node::Empty,
        }
    }

    pub fn value(&self) -> Value {
        self.value
    }

    pub fn left(&self) -> &Node {
        &self.left
    }

    pub fn right(&self) -> &Node {
        &self.right
    }
}

impl TreeElement for Branch {
    fn contains(&self, value: Value) -> bool {
        match self.value.cmp(&value) {
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Less => self.right.contains(value),
            std::cmp::Ordering::Greater => self.left.contains(value),
        }
    }

    fn insert(&mut self, value: Value) {
        if value < self.value {
            self.left.insert(value);
        } else {
            self.right.insert(value);
        }
        self.size += 1;
    }

    fn size(&self) -> usize {
        self.size
    }
}
