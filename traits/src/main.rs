mod basket;
mod stack;
mod container;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(container: &mut T, s: String) {
    container.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("hi there"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("hi")]);
    let mut s2 = Stack::new(vec![1, 2, 3]);

    add_string(&mut b1, String::from("hi"));
    add_string(&mut s1, String::from("hi"));
    // add_string(&mut s2, String::from("hi")); // will not work because it's not type string
}
