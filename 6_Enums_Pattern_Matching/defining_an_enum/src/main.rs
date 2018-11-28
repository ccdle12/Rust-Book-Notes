// IpAddrKind uses two enum variants:
// V4 and V6.
enum IpAddrKind {
    V4,
    V6,
}

// Enum variants can be associated
// with a value, which makes the
// struct below unecessary.
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

// Use IpAddrKind in a struct.
// Since we specified the field
// as IpAddrKind, any it's variant
// can be used.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Enum variant with multiple types, making
// the value more strict in the case of V4.
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Message is an enum with several variants.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

// Like structs, Enums can have methods.
impl Message {
    fn call(&self) {
        println!("hello")
    }
}

fn main() {
    // We have defined two instances of the
    // varient enum IpAddrKind.
    //
    // Both V4 and V6 can be passed into the
    // function route(ip_type: IpAddrKind)
    // since V4 and V6 are variants of the same
    // enum.
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // Since route takes in IpAddrKind
    // we can pass it V4 or V6.
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Create IpAddr structs.
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Create the enum variants with an associated value.
    let _home2 = IpAddrKind2::V4(String::from("127.0.0.1"));
    let _loopback2 = IpAddrKind2::V6(String::from("::1"));

    // Create the enum with multiple values.
    let _home3 = IpAddrKind3::V4(127, 0, 0, 1);
    let _home3 = IpAddrKind3::V6(String::from("127.0.0.1"));

    let m = Message::Write(String::from("Hello, World!"));
    m.call();

    // Option Enum
    // Used to improve Null pointer references.
    let name = String::from("Dominic");

    // Attempt to print letter 8 that does not exist.
    println!(
        "Character at index 8: {}",
        match name.chars().nth(8) {
            // name.chars() is an iterator and nth(8) is the 8th index position.
            Some(c) => c.to_string(), // If there is Something there, return the character as a string,
            None => "No Character at index 8!".to_string(), // If there is no character, return the message,
                                                            // handling the null pointer exception.
        }
    );

    // This will fail since we need to convert options to the its actual type.
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // let sum = x + y;
}

// route has a parameter of IpAddrKind.
fn route(ip_type: IpAddrKind) {}
