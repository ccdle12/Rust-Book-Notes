// TODO: Continue from ### Using External Packages

/// Module Sound, used as crate?
/// They are currently Private so cannot be used.
/// Mods are the Privacy Boundaries in Rust. If we want
/// a struct of function to be private, we wrap it in a
/// Module.
mod sound; // import the sound mod from it .rs file. The .rs file just has implementation.

/// Plant is a private module.
mod plant {
    /// Vegetable a public struct, with a public field name.
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

/// Public Enum.
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

/// The use keyword will allow the specific path of a crate
/// to be used in main.
/// Apparently this is a unsafe feature?
// use crate::sound::instrument;
/// We can use self to refer to the mod, self is essentially
/// a relative path.
/// NOTE: Only calling the module path is idiomatic if we are just
/// calling a function.
use self::sound::instrument;

/// NOTE: Calling structs and enums, its idiomatic to specify the path
/// all the way to the struct/enum.
use std::collections::HashMap;

/// Using the use keyword in another mod.
mod performance_group {
    // AGAIN: crate is unsafe?
    // use crate::sound::instrument;

    // Relative paths work.
    // Using "pub" will re-export the variable.
    pub use super::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

/// Bringing two types of the same name in to scope.
/// In this case it's better to use the scope of each crate
/// that way the compiler will know which Result type to use.
use std::fmt;
// use std::io;

fn function1() -> fmt::Result {
    Ok(())
}

// fn function2() -> io::Result<()> {
//     Ok(())
// }
//

/// There is another technique to prevent bringing two types of the same
/// name into scope.
/// The 'as' keyword will import a type and rename it.
use std::io::Result as IoResult;
fn function2() -> IoResult<()> {
    Ok(())
}

/// Need to declare the user of an external crate.
extern crate rand;
/// Import the rand crate and use it.
use rand::Rng;

fn main() {
    // Think of '::' as '/' in file paths.
    sound::instrument::clarinet();

    // This will fail because breath_in() is inside sound which is
    // still a private module.
    // sound::breath_in();

    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("Cucumber");
    println!("Vegetable name: {}", v.name);

    let soup = menu::Appetizer::Soup;
    let salad = menu::Appetizer::Salad;

    instrument::clarinet();

    performance_group::clarinet_trio();

    // Using the public re-exported variable.
    performance_group::instrument::clarinet();

    let mut map = HashMap::new();
    map.insert(1, 2);

    // Use the imported rand module.
    // let secret_number = rand::thread_rng().gen_range(1, 101);
    let secret_number = rand::thread_rng().gen_range(1, 101); // gen_range is inclusive in lower bound and exclusive on upper bound.
}
