#[derive(Debug)] // allows to print the enum variants.
enum USState {
    Alabama,
    Alaska,
}

// Enum of US Coins
#[derive(Debug)] // allows to print the enum variants.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn main() {
    // Matches a Some(0u8) value, if it equals
    // 3 it prints and catches everything else.
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("NOT THREE"),
    }

    // Using if let pattern.
    // Syntatic sugare for match, it does not
    // do the exhaustive match for all possibilties.
    if let Some(3) = some_u8_value {
        println!("Three")
    }

    // Short hand if let else.
    // Match the coin variable to an inner scoped
    // let Coin::Quarter(state)
    let mut _count = 0;
    let coin = Coin::Quarter(USState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State from quarter {:?}", state);
    } else {
        _count += 1;
    }
}
