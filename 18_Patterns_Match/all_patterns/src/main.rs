fn main() {
    // Matching literals, useful on a particular concrete value.
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("anything"),
    };

    // Matching Some.
    // Will return _, since x is None.
    let x = None;
    let _y = Some(5);

    match x {
        Some(50) => println!("fifty"),
        Some(2) => println!("2"),
        _ => println!("no match"),
    };

    // Multiple Patterns.
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("no match"),
    }
}
