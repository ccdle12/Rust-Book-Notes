fn main() {
    // Create a string.
    let s1 = String::from("hello");

    // 1. Pass the reference of the string to the fn. 
    // '&' allows passing a reference of the variable, 
    // meaning it does not change ownership.

    // 2. Example:
    // s: ptr -> s1: ptr, len, capacity -> index:0, value: h
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 3. This will fail since we are trying to modify a reference.
    // change(&s1)

    // 4. Mutable References
    // This will allow us to pass a mutable reference, meaning
    // the reference can be modified.
    let mut s2 = String::from("hello");
    change(&mut s2);

    println!("The string s2 is now: {}", s2);

    // 6. Rust does not allow multiple borrowing of a mutable reference
    // in the same scope. This prevents race conditions at compile time.
    // let r1 = &mut s2;
    // let r2 = &mut s2;

    // 7. Rust allows multiple not mutable references because users cannot change
    // the reference.
    let _r1 = &s2;
    let _r2 = &s2;
    // let r3 = &mut s; This fails because we cannot have mutable references
    // in the same scope immutable references. This will change the immutable
    // reference values at compile time.

    // 8. Dangling References
    // let reference_to_nothing = dangle(); // This will fail because ownership
    // was not changed. 

    // 9. Dangling References Solution
    let tranf_ownership_str = not_dangling();
    println!("{}", tranf_ownership_str)
}

// 8. Dangling Reference function
// This function will not return anything due to the return type being a reference
// to a string, but the ownership of the string has not successfully moved out 
// of scope.
// fn dangle() -> &String {
//     let s = String::from("hello");
    
//     &s
// }

// 9. Transfers ownership of the String, not making it a dangling reference. 
fn not_dangling() -> String {
    let s = String::from("hello");

    s
}

// 2. calculate_length expects a reference to a String.
fn calculate_length(s: &String) -> usize {
    // `Drop` is never called because no ownership, has
    // taken place.
    s.len()
}

// 5. This function will fail because it is trying modify
// a reference. References are IMMUTABLE, since it is
// only a reference.
// fn change(s: &String) {
    // s.push_str(" world!");
// }

// 5. This function will be able to modify the string since
// it is expecting a mutable string.
fn change(s: &mut String) {
    s.push_str(" , world!")
}
