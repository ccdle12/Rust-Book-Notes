fn main() {
    // Shallow copy, it copies the ptr, length and capacity. 
    // It invalidates the first reference after the copy.
    // When the variables are moved out of scope, there will
    // be no double memory release error since the first 
    // variable was invalid after the copy.

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

    println!("s3: {}, s4: {}", s3, s4)
}
