use super::node_leaf::NodeLeaf;
use crate::rope::RopeNode;

pub enum Node {
    Leaf(NodeLeaf),
}

impl RopeNode for Node {
    fn new() -> Self {
        Node::Leaf(NodeLeaf::new())
    }

    fn from_str(string: &str) -> Self {
        let mut node = Node::new();
        node.append(string);
        node
    }

    fn insert(&mut self, index: usize, string: &str) {
        match self {
            Node::Leaf(node) => node.insert(index, string),
        }
    }

    fn append(&mut self, string: &str) {
        match self {
            Node::Leaf(node) => node.append(string),
        }
}

    fn char(&self, index: usize) -> char {
        unimplemented!()
    }

    fn len(&self) -> usize {
        match self {
            Node::Leaf(node) => node.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_node() {
        let node = Node::new();
        assert_eq!(node.len(), 0);
    }

    #[test]
    fn create_node_from_str() {
        let node = Node::from_str("Hello World");
        assert_eq!(node.len(), "Hello World".as_bytes().len());
    }
}
