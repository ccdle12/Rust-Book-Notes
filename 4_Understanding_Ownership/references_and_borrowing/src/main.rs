fn main() {
    // Create a string.
    let s1 = String::from("hello");

    // Pass the reference of the string to the fn. 
    // '&' allows passing a reference of the variable, 
    // meaning it does not change ownership.

    // Example:
    // s: ptr -> s1: ptr, len, capacity -> index:0, value: h
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // This will fail since we are trying to modify a reference.
    // change(&s1)
}

// calculate_length expects a reference to a String.
fn calculate_length(s: &String) -> usize {
    // `Drop` is never called because no ownership, has
    // taken place.
    s.len()
}

// This function will fail because it is trying modify
// a reference. References are IMMUTABLE, since it is
// only a reference.
// fn change(s: &String) {
    // s.push_str(" world!");
// }