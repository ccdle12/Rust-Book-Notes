fn main() {
    let x = 5;

    // match acts like if statements
    match x {
        1 => println!("It's 1"),
        2 => println!("2"),
        // catches all
        _ => println!("Not sure what it is")
    };

    // Useful for converting types
    let converted = match x {
        5 => "five",
        _ => "Nil"
    };
    println!("x converted: {}", converted);

    // Using OR statements with multiple patterns to match
    match x {
        4 | 5 => println!("four or five"),
        _ => println!("No match")
    };
    
}
