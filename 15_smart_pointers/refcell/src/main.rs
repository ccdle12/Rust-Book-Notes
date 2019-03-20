fn main() {
    // Refcell represents single ownerhsip over the data it holds but it allows
    // for unsafe referencing.
    // The borrowing rules of Rust are:
    // * You can have either one mutable reference or one immutable reference
    //   NOT BOTH!
    // * References must always be valid
    //
    // Refcell allows for both references on a single variable. The invariants
    // for Box<T> is checked at compile time, preventing the double reference.
    // The invariants for Refcell<T> is checked at runtime, if something breaks
    // the program will panic and exit.
    //
    // RefCell is useful when you are sure your code respects the borrowing
    // rules as the Rust compiler will be conservative.
    //
    // !!!! Recap on Smart Pointers: !!!!
    // * Rc<T> enables multiple owners of the same data, Box<T> and RefCell<T>
    //   have single owners.
    // * Box<T> allows immutable and mutable borrows at compile time.
    // * Rc<T> only allows immutable borrows at compile time.
    // * RefCell<T> allows mutable borrows checked at *run time*, you can
    //   mutate the value inside a RefCell<T> even when it is immutable.

    // An example of the mutable and immutable borrow.
    // This will return compile time errors of multiple borrows.
    // let x = 5;
    // let y = &mut x;

    // TODO: (ccdle12) continue from "A use case for interior mutability: Mock
    // Objects.
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You have used up over 75% of your quota!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // We can use Refcell here.
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // Initialize a RefCell with a vector.
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // Here is the issue, we have an immutable reference from the Messenger
        // trait, but the MockMessenger implementation requires a mutable
        // reference.
        fn send(&self, message: &str) {
            // Here we can use borrow_mut to be able to use a mut borrow from
            // the RefCell.
            self.sent_messages.borrow_mut().push(String::from(message))
        }
    }

    // Explanation:
    // * MockMessenger implements Messenger and it's send fn pushes the
    // received message to a RefCell::new(vec![]).
    //
    // * When set value is called, it calls send according to the value, pushes
    // the message to the sent_messages vector.
    //
    // * send in MockMessenger *mutably* borrows self via calling sent_messages
    // , this is because the implementation of Messenger only allows *immutable*
    // borrow of self. We use RefCell since we know this respects the borrowing
    // rules and we don't want to change the Messenger trait to be mutable.
    #[test]
    fn it_sends_over_75_percent_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
