fn main() {
    // 1. String slices are a reference to a part of a string. 
    let s = String::from("Hello world!");

    // 2. The slice returns a reference of the string index 
    // positions. (NOTE: does not include the last index postion)
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    // 3. ..= means include the last index position as well. 
    let hello = &s[0..=4];
    let world = &s[6..=10];

    println!("{} {}", hello, world);

    // 4. Shorthand for referencing from 0. 
    let hello = &s[..2];

    println!("{} {}", hello, world);

    // 5. Shorthand for referencing until the end of the string. 
    let hello = &s[2..];
    println!("{} {}", hello, world);

    // 6. Shorthand slice of the whole string.
    let hello_world = &s[..];
    println!("{}", hello_world);

    // 7. Get first word.
    let hello = first_word(&s);
    println!("{}", hello);

    // 8. Slices can also be used for other types as as well.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..=3];
    println!("{}", slice[2]);
}

// 7. Function that finds the first word in a string.
// Returns a pointer to a str because the string has 
// not changed ownership.
fn first_word(s: &String) -> &str {
    // Convert the string to a slice of bytes.
    let bytes = s.as_bytes();

    // Iterate over the slice of bytes. 
    // If there is an empty byte, return a slice 
    // up to the point i.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // The input string was 1 word anyway, return it.
    &s[..]

}
