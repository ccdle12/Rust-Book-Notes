enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
fn main() {
    // Box<T> stores data on the heap, enables the movement of the pointer on the stack without
    // copying the data on the heap.
    let b = Box::new(5);
    println!("b = {}", b);

    // cons list is a datastructure that is common in functional programming. It is used enable
    // recursive types like Box.
    // This works because with cons variant has a {Cons, List}, a Cons referning another List.
    // Without the Box type as the pointer on the stack, Rust does not know much space to
    // allocate. Using the Box type, Rust knows the size needed for a pointer.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
