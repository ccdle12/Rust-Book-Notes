use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Without the Deref trair, the compiler can only dereference &.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // returns a reference to the value we want to access.
        &self.0
    }
}

fn hello(msg: &str) {
    println!("Hello {}", msg)
}

fn main() {
    // We reference and borrow the i32 from x and assign to y.
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // The reason this fails is because we need to deference the y, to point the value it is
    // refernencing.
    // assert_eq!(5, y);
    assert_eq!(5, *y);

    // Here we are using Box instead of a reference to x.
    let x = 10;
    let y = Box::new(x);
    assert_eq!(10, x);
    assert_eq!(10, *y);

    let x = 2;
    let y = MyBox::new(x);
    assert_eq!(2, x);
    // Behind the scenes, rust substitutes * witht he defer function.
    assert_eq!(2, *y);

    // Deref coercion allows easier referencing, implemented via Deref trait.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // If rust didn't implement deref coercion,m we have to right the above as...
    // hello(&(*m)[..])
}
