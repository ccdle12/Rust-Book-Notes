use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

// Using the drop trait, it will realse the object from memory as soon as it leaves scope.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    // Using the Drop trait to release objects from memory when moved out of scope.
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // This fails because we cannot explicitly call the drop trait.
    // The reason being, this would cause a double free error, since the drop trait would call drop
    // automatically once the object leaves scope.
    // We can use the std::mem::drop function to release memory early, the reason we want to do
    // this might be in place of mutex locks and we want to allow another program access.
    // c.drop();
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
