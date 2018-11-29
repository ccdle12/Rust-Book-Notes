#[derive(Debug)] // allows to print the enum variants.
enum USState {
    Alabama,
    Alaska,
}

// Enum of US Coins
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

// Use match to handle the enum argument.
fn value_in_cents(coin: Coin) -> u32 {
    // Matches in Rust are exhaustive, it forces the handling
    // of every case.
    match coin {
        Coin::Penny => {
            println!("Its a penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// Plus one takes ownership of an Option<i32>
// and returns an Option<i32> passing ownership.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // Pass a Alaskan Quarter to value_in_cents.
    value_in_cents(Coin::Quarter(USState::Alaska));

    // Call plus one with Some and None values.
    let six = plus_one(Some(5));
    println!("{:?}", six);

    let none = plus_one(None);
    println!("{:?}", none);

    // The character _ will catch all other cases.
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("Caught all other cases"),
    }
}
