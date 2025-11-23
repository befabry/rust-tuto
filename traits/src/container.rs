/// A generic container trait - similar to interfaces + abstract classes
///
/// TRAITS = RUST'S VERSION OF:
/// - ✅ Interfaces (define required methods)
/// - ✅ Abstract classes (can have default implementations)  
/// - ✅ Mixins (can be composed together)
/// - ❌ No fields/data (only methods)
///
/// This trait defines a basic container API that can work with any type T
pub trait Container<T> {
    fn get(&mut self) -> Option<T>;
    fn put(&mut self, item: T);
    fn is_empty(&self) -> bool;
}
