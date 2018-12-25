fn main() {
    // Definiing a vector.
    let _v: Vec<i32> = Vec::new();

    // Inferring a vector type.
    let _z = vec![1, 2, 3];

    // Pushing to a vector.
    let mut a = vec![1, 2, 3];
    a.push(4);
    a.push(5);

    out_of_scope();

    // Reading from vectors
    let b = &a[1];
    println!("Index 1 of vector a is: {}", b);

    match a.get(1) {
        Some(first) => println!("The first index is: {}", first),
        None => println!("The is no first element"),
    };

    // Accessing an index that doens't exist.
    // let _doesnt_exist = &a[100];

    // Cannot have a reference and add to a vector in the same scope.
    // This is because adding a new element, might lead to the vector
    // resizing the array in memory and needing to copy over the values,
    // thus loosing the pointer to memory.
    // let first = &a[0];
    // a.push(9);

    // vector iteration.
    for i in &a {
        println!("Value of i in a: {}", i);
    }

    // Iterate over a mutable vector and change each item.
    let mut c = vec![50, 100, 150];
    for i in &mut c {
        *i += 50;
    }

    // Print the results, should be [100, 150, 200].
    for i in c {
        println!("Updated results in c: {}", i);
    }

    // Using an Enum to be able to store multiple types
    // in a vector.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.32),
        SpreadsheetCell::Text(String::from("hello")),
    ];
}

fn out_of_scope() {
    let _z = vec![1, 2, 3];
} // The vector z is freed once out_of_scope completes.
