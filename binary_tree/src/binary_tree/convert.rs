use super::{BinaryTree, Value};

impl From<Vec<Value>> for BinaryTree {
    fn from(values: Vec<Value>) -> Self {
        let mut tree = BinaryTree::new();
        for value in values {
            tree.insert(value);
        }
        tree
    }
}

impl From<BinaryTree> for Vec<Value> {
    fn from(tree: BinaryTree) -> Self {
        let mut values = Vec::with_capacity(tree.size());
        tree.root.push_to_vec(&mut values);
        values
    }
}
