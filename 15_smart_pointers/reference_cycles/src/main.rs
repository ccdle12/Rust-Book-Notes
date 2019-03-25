use std::cell::RefCell;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};

fn main() {
    // EXAMPLE:
    // Cyclical references.
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // Creating a Tree Data Structure.
    // Leaf is a node that contains the value 3 and no children it is
    // referencing.
    //
    // Here we update the Node to include a 'parent field' which stores a weak
    // reference. This is so that we don't get the issues of reference cycles,
    // by having a weak reference, leaf doesn't own branch but can reference it.
    // If the branch is dropped from memory leaf will eb dropped as well.
    // If leaf is dropped from memory, this does not affect it's parent.
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // The reference to the parent is currently None.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // We create the node branch and clone leaf it it's children. This means
    // that leaf has two owners, leaf and branch. We can access leaf via branch
    // by 'branch.children'. Currently leaf has no knowledge of branch as it's
    // parent.
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Downgrade creates a weak pointer to &branch.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // SUMMARY:
    // Box<T> has a known size and points to data allocated on the heap.
    // Rc<T> keeps track of the number of references to the data on the heap.
    // RefCell<T> allows immutable and mutable references of data.
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
