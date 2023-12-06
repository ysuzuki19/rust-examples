use super::{branch::Branch, common::TreeElement};

type Value = i32;

#[derive(Default, Clone)]
pub enum Node {
    Branch(Box<Branch>),
    #[default]
    Empty,
}

impl TreeElement for Node {
    fn contains(&self, value: Value) -> bool {
        match self {
            Node::Branch(branch) => branch.contains(value),
            Node::Empty => false,
        }
    }

    fn insert(&mut self, value: Value) {
        match self {
            Node::Branch(branch) => {
                branch.insert(value);
            }
            Node::Empty => {
                *self = Node::Branch(Box::new(Branch::new(value)));
            }
        }
    }

    fn size(&self) -> usize {
        match self {
            Node::Branch(branch) => branch.size(),
            Node::Empty => 0,
        }
    }
}

impl Node {
    pub fn push_to_vec(&self, vec: &mut Vec<Value>) {
        match self {
            Node::Branch(branch) => {
                branch.left().push_to_vec(vec);
                vec.push(branch.value());
                branch.right().push_to_vec(vec);
            }
            Node::Empty => {}
        }
    }
}
