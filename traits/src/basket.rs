use crate::container::Container;

pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket { item: Some(item) }
    }
}

/// Implements the Container trait for Basket
///
/// - Makes Basket<T> a valid Container<T> that can be used polymorphically
/// - get() uses Option::take() to remove and return the item (leaves None)
/// - put() replaces any existing item (like swapping items in a basket)
/// - is_empty() checks if the basket currently holds an item
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
