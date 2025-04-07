use super::container::Container;

pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket {
            item: Some(item)
        }
    }
}

// first T is to pass in the arg for the type
impl<T> Container<T> for Basket<T> {
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}