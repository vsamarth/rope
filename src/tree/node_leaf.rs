use crate::rope::RopeNode;

#[derive(Debug)]
pub struct NodeLeaf {
    buffer: Vec<u8>,
}

impl RopeNode for NodeLeaf {
    fn new() -> Self {
        NodeLeaf { buffer: Vec::new() }
    }

    fn from_str(string: &str) -> Self {
        NodeLeaf {
            buffer: string.as_bytes().to_vec(),
        }
    }

    fn insert(&mut self, index: usize, string: &str) {
        unimplemented!()
    }
    
    fn append(&mut self, string: &str) {
        self.buffer.extend(string.as_bytes());
    }

    fn char(&self, index: usize) -> char {
        let c: char = self.buffer[index] as char;
        c
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}
