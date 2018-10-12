fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    // EXPRESSIONS VS STATEMENTS
    let _x = 5;

    // This should fail and not return a value
    // to y. This is because have a ';' at the 
    // end, signifies a statement.
    // let y = {
    //     let x = 3;
    //     x + 1;
    // };

    // This should return 4 and assign it to y. 
    // This is because the last command does not
    // have a ';' making it a statement, or precisely
    // a return statement.
    let y = {
        let x = 3;
        x + 1
    };
    println!("y is: {}", y);

    // CALLING A FUNCTION
    let z = five();
    println!("z is: {}", z);

    let a = plus_one(10);
    println!("a is: {}", a);
}

fn another_function(x: i32, y: i32) {
    println!("x: {}", x);
    println!("y: {}", y);
}

// This function declares a return type '-> i32'.
// Because the last statement does not have a ';',
// it becomes the return value ("statement").
// A function can return early using the 'return' keyword.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}