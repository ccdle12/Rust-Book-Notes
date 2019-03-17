enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    // Reference counting is another Smart Pointer that counts the number of
    // variables that point to a given value. This is specifically useful in
    // graph data structures a node can be owned by multiple edges that point
    // to it.

    // This fails to compile because b owns a and c own a as it has already
    // been moved.
    // We could change the definition of Cons to hold references but then we
    // would need to specify lifetimes. Every element in the list would live
    // as long as the entire list.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // We can use Rc::new() to create a reference counter and using Rc::clone()
    // this does not create a deep copy but only increments the reference
    // counter.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
