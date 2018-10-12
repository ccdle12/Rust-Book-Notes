fn main() {
    // Shallow copy, it copies the ptr, length and capacity. 
    // It invalidates the first reference after the copy.
    // When the variables are moved out of scope, there will
    // be no double memory release error since the first 
    // variable was invalid after the copy.

    // Rust uses a annotation `Drop` which will release 
    // memory associated with the variable, when the variable
    // is moved out of scope.
    // This is essentially a Resource Acquisition Is Initialization (RAII)
    // pattern.

    let s1 = String::from("hello");
    let s2 = s1;

    // This will fail.
    // println!("{}", s1)

    println!("{}", s2);

    // Deep Copy, it will copy the String object in the heap 
    // and also the pointer on the stack. This is an expensive
    // operation.
    let s3 =  String::from("hello");
    let s4 =  s3.clone();

    println!("s3: {}, s4: {}", s3, s4);

    // Stack Only Data: Copy
    // Integers can make copies without invalidating the previous
    // assignment since the size of an integer is known at compile
    // time. The is entirely stored on the stack, and copying
    // values is quick.

    // Rust uses an annotation `Copy` trait that is placed on types
    // that are stored on the stack. If the `Copy` trait is used in
    // annotation, then an older copy is still valid. `Copy` cannot
    // be used with the trait `Drop`.

    // Types that are `Copy`: 
    // * integers
    // * bools
    // * floating points
    // * chars
    // * Tuples only if they contain Copy types
    //      * (i32, i32) = valid
    //      * (i32, Striing) = invalid
    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);


    // Ownership and Functions
    let s = String::from("hello");
    takes_ownership(s);

    // This will fail since we have moved `let s`
    // to `fn takes_ownership(received_s: String)`, meaning we have
    // made a shallow copy - copying the ptr, length and capacity to 
    // `received_s`, therefore invalidating the previous variable in
    // the heap.
    // println!("{}", s);

    // This won't fail because we are copying types that have the
    // `Copy` annotation, meaning these variables are kept in the stack.
    // Copying them does not invalidate previous variables in memory.
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // Return Values and Scope
    // Intitalizes a string and transfers ownership to s5.
    let s5 = gives_ownership();

    // Initialize a string in this scope.
    let s6 = String::from("world");

    // Transfer ownership of s6 to a fn call and return it
    // back to s7.
    let s7 = takes_and_gives_back(s6);
}

fn takes_ownership(received_s: String) {
    println!("{}", received_s);
}

fn makes_copy(received_integer: i32) {
    println!("{}", received_integer);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    
    // Return s by not appending ';'
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}
