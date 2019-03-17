fn main() {
    // Refcell represents single ownerhsip over the data it holds but it allows
    // for unsafe referencing.
    // The borrowing rules of Rust are:
    // * You can have either one mutable reference or one immutable reference
    //   NOT BOTH!
    // * References must always be valid
    //
    // Refcell allows for both references on a single variable. The invariants
    // for Box<T> is checked at compile time, preventing the double reference.
    // The invariants for Refcell<T> is checked at runtime, if something breaks
    // the program will panic and exit.
    //
    // RefCell is useful when you are sure your code respects the borrowing
    // rules as the Rust compiler will be conservative.
    //
    // !!!! Recap on Smart Pointers: !!!!
    // * Rc<T> enables multiple owners of the same data, Box<T> and RefCell<T>
    //   have single owners.
    // * Box<T> allows immutable and mutable borrows at compile time.
    // * Rc<T> only allows immutable borrows at compile time.
    // * RefCell<T> allows mutable borrows checked at *run time*, you can
    //   mutate the value inside a RefCell<T> even when it is immutable.
}
