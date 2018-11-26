enum IpAddrKind {
    V4,
    V6,
}

// Enum variant with values.
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

// Use IpAddrKind in a struct.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Enum variant with multiple types.
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Since route takes in IpAddrKind
    // we can pass it v4 or v6.
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Create IpAddr structs.
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Enum variant with values.
    let home2 = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrKind2::V6(String::from("::1"));

    let home3 = IpAddrKind3::V4(127, 0, 0, 1);
    let home3 = IpAddrKind3::V6(String::from("127.0.0.1"));
}

// route has a parameter of IpAddrKind.
fn route(ip_type: IpAddrKind) {}