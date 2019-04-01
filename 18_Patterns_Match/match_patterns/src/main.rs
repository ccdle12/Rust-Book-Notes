fn main() {
    let colour: Option<&str> = Some("Blue");

    if let Some(c) = colour {
        println!("Colour has a value of {}", c);
    } else {
        println!("Colour has no value");
    }

    // Using let pattern matching in a loop.
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // Using pattern match in for loop.
    let v = vec!["a", "b", "c"];

    // enumerate() will produce the (index, value).
    for (i, v) in v.iter().enumerate() {
        println!("{} is at index {}", i, v);
    }

    // destructuring.
    let (x, y, z) = (1, 2, 3);

    // Function paramters.
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("co-ordinates: {}, {}", x, y);
}
