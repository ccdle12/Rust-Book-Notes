struct Point {
    x: i32,
    y: i32,
}

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

    // Matching ranges with values.
    let x = 5;
    match x {
        1...5 => println!("one through five"),
        _ => println!(""),
    }

    // Matching ascii.
    let x = 'c';
    match x {
        'a'...'j' => println!("early ascii letter"),
        'k'...'z' => println!("late ascii letter"),
        _ => println!("something else"),
    }

    // Destructuring structs.
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
