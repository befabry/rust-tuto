mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("hello"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("hello"), String::from("world")]);
    let s2 = Stack::new(vec![1, 2, 3]);

    add_string(&mut b1, String::from("hi"));
    add_string(&mut s1, String::from("there"));
}
