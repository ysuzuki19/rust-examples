type Value = i32;

pub struct Branch {
    value: Value,
    size: usize,
    left: Node,
    right: Node,
}

impl Branch {
    fn new(value: Value) -> Branch {
        Branch {
            value,
            size: 1,
            left: Node::Empty,
            right: Node::Empty,
        }
    }

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

#[derive(Default)]
pub enum Node {
    Branch(Box<Branch>),
    #[default]
    Empty,
}

impl Node {
    pub fn contains(&self, value: Value) -> bool {
        match self {
            Node::Branch(branch) => branch.contains(value),
            Node::Empty => false,
        }
    }

    pub fn insert(&mut self, value: Value) {
        match self {
            Node::Branch(branch) => {
                branch.insert(value);
            }
            Node::Empty => {
                *self = Node::Branch(Box::new(Branch::new(value)));
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Node::Branch(branch) => branch.size(),
            Node::Empty => 0,
        }
    }

    pub fn push_to_vec(&self, vec: &mut Vec<Value>) {
        match self {
            Node::Branch(branch) => {
                branch.left.push_to_vec(vec);
                vec.push(branch.value);
                branch.right.push_to_vec(vec);
            }
            Node::Empty => {}
        }
    }
}
