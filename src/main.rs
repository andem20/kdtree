fn main() {
    let mut tree = KDTree::new();
    let values = vec![[5, 200], [10, 1200], [22, 6], [1, 8], [2, 13]];
    tree.insert_values(values);

    println!("{:?}", tree.inorder_traversal());
}

type NodeType<T, const K: usize> = Option<Box<Node<T, K>>>;

#[derive(Debug)]
struct Node<T, const K: usize> {
    value: [T; K],
    left: NodeType<T, K>,
    right: NodeType<T, K>,
}

impl<T, const K: usize> Node<T, K>
where
    T: std::cmp::PartialOrd + Clone,
{
    fn new(value: [T; K]) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct KDTree<T, const K: usize> {
    root: NodeType<T, K>,
}

impl<T, const K: usize> KDTree<T, K>
where
    T: std::cmp::PartialOrd + Clone,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert_value(&mut self, value: [T; K]) {
        KDTree::insert_record(&mut self.root, value, 0);
    }

    pub fn insert_values(&mut self, values: Vec<[T; K]>) {
        values
            .into_iter()
            .for_each(|value| self.insert_value(value));
    }

    fn insert_record(parent: &mut NodeType<T, K>, value: [T; K], depth: usize) {
        let dimension = depth % K;
        if let Some(parent) = parent {
            if value[dimension] < parent.value[dimension] {
                KDTree::insert_record(&mut parent.left, value, depth + 1);
            } else if value[dimension] > parent.value[dimension] {
                KDTree::insert_record(&mut parent.right, value, depth + 1);
            }
        } else {
            *parent = Some(Box::new(Node::new(value)));
        }
    }

    fn inorder_traversal(&self) -> Vec<[T; K]> {
        let mut result = vec![];
        self.traverse(&self.root, &mut result);
        return result;
    }

    fn traverse(&self, node: &NodeType<T, K>, result: &mut Vec<[T; K]>) {
        if let Some(node) = node {
            self.traverse(&node.left, result);
            result.push(node.value.clone());
            self.traverse(&node.right, result);
        }
    }
}
