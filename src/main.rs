fn main() {
    let mut tree = kdtree::KDTree::new();
    let values = vec![
        [5, 200, 2],
        [10, 1200, 1],
        [22, 6, 399],
        [10, 80, 200],
        [2, 13, 1],
    ];
    tree.insert_values(values);

    println!("{:?}", tree.inorder_traversal());
}
