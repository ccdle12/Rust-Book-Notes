// Tuple Struct.
struct Dimensions(u32, u32);

// Using a Struct.
// Adding the derive debug annotation
// to allow the struct to be printed.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Calculate the area of a triangle.
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Refactoring using struct Tuples.
    let rect1 = Dimensions(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // Refactoring using struct.
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // Passing an immutable pointer (borrowing) since
        // we want to use the struct again in main.
        area3(&rect2)
    );
    println!("Using rect2 again in main after borrowing: {}", rect2.height);

    println!("Printing the struct Rectangle: {:?}", rect2)
}

// Naive implementation.
fn area(width: u32, height: u32) -> u32 {
    // No ;, means return
    width * height
}

// Using struct Tuples.
fn area2(dimensions: Dimensions) -> u32 {
    dimensions.0 * dimensions.1
}

// Using struct.
// Receive Rectangle as an immutable pointer (borrowing),
// because want main to retain ownership. 
// This allows rectangle to be continued
// to be used in main.
fn area3(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
