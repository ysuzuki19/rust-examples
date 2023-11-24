use crate::binary_tree::BinaryTree;

mod binary_tree;

fn main() {
    let mut tree = BinaryTree::new();
    assert_eq!(tree.size(), 0);
    assert!(!tree.contains(4));

    tree.insert(4);
    assert!(tree.contains(4));
    assert_eq!(tree.size(), 1);

    tree.insert(5);
    assert!(tree.contains(4));
    assert!(tree.contains(5));
    assert_eq!(tree.size(), 2);

    let v = vec![4, 5, 6, 7, 8, 9, 10];
    let tree = BinaryTree::from(v.clone());
    assert_eq!(tree.size(), 7);
    assert!(tree.contains(4));
    assert!(tree.contains(5));
    assert!(tree.contains(6));
    assert!(tree.contains(7));
    assert!(tree.contains(8));
    assert!(tree.contains(9));
    assert!(tree.contains(10));
    assert!(!tree.contains(3));
    let r: Vec<_> = tree.into();
    assert_eq!(v, r);
}
