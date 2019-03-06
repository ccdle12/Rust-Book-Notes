// This folder was generated using:
// $ cargo new adder --lib

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value must be between 1 and 100: {}", value)
        }

        Guess { value }
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("FAIL!");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };

        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_not_equal() {
        assert_ne!(2, 3);
    }

    // NOTE: assert_eq! and assert_ne! prints the values, therefore both values must use PartialEq
    // and Debug trait.

    // Testing for errors, using should_panic.
    #[test]
    #[should_panic]
    fn test_for_errors() {
        Guess::new(200);
    }

    #[test]
    fn handling_errors() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // an expensive test
    }
}
