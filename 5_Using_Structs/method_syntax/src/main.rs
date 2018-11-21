// Using a Struct.
// Adding the derive debug annotation
// to allow the struct to be printed.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation of the Rectangle struct.
// Methods that belong to that struct.
impl Rectangle {
    // &self to borrow the data in the struct
    // NOT take ownership.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Associated Functions, has no refernce to the struct.
    // Can be called like String::from("hello")
    // Acts as a constructor.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// A struct can have multiple implementation blocks,
// there's no reason to separate them but it's valid.
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Use the method of the Rectangle struct.
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    // Create a pointer to rect1.
    let rect2 = &rect1;

    // Use the pointer to call area().
    println!(
        "The area of the rectangle is {} square pixels",
        rect2.area()
    );

    // Adding a second method to the Rectangle.
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };
    let _rect5 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect4));

    // Construct Rectangle using the associated function.
    let rect6 = Rectangle::square(10);
    println!("rect6 area is: {}", rect6.area());
}
