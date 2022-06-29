pub trait RopeNode {
    fn new() -> Self;
    fn from_str(string: &str) -> Self;

    fn insert(&mut self, index: usize, string: &str);
    fn append(&mut self, string: &str);

    fn char(&self, index: usize) -> char;
    fn len(&self) -> usize;
}
