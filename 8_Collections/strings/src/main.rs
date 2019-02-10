fn main() {
    let data = "initial contents";
    let s = data.to_string();

    // This method can also work on a literal string.
    let s = "Hello, World!".to_string();
    println!("{}", s);

    // String initialisation.
    let mut a = String::from("initialised");
    println!("{}", a);

    // Strings, like vectors can have other strings
    // pushed to it.
    let a2 = " pushed";
    a.push_str(a2);
    println!("After pushing a string: {}\n", a);

    // Can even push single characters to a String.
    // Using the "push" function.
    let mut b = String::from("lo");
    let b2 = 'l';
    b.push(b2);
    println!("Pushed character: {}", b);

    // Classic String Concatenation.
    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s4 = "!";

    // Rust requires one of the Strings to change ownership
    // and the others can be borrowed.
    let s3 = s1 + &s2 + &s4;

    println!("S3: {}", s3);
    println!("S2: {}", s2);
}
