pub trait Container<T> {
    // abstract methods (no method body)
    fn get(&mut self) -> Option<T>;
    fn put(&mut self, item: T);
    fn is_empty(&self) -> bool;
}