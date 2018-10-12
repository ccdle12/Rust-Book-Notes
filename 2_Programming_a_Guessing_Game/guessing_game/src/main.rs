// Using rand as an external crate
// use as rand::
extern crate rand;

use std::io;

// Use Ordering to compare guesses.
use std::cmp::Ordering;

// Use Rng methods found in rand
// Rng is random number generator
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1, 101); // gen_range is inclusive in lower bound and exclusive on upper bound.

    // infinite loop
    loop {
        // prompts the user to input their guess
        println!("Please input your guess: ");

        // let is used to create a variable
        // mut (mutable) allows the variable to be changed
        // all variables are immutable by default
        let mut guess = String::new();

        // using standard input, read the whole line,
        // write input to the reference of guess
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // expect will catch errors returned
        
        // we are shadowing guess and converting it to a uint32
        // 1. we need to explicitly declare the int type after semi-colons (":")
        // 2. we need to trim to the string and remove whitespaces, when the user presses 'enter' when guessing
        // it adds a new line '/n'
        // 3. we need to catch any errors when parsing to int, for example, we can't parse @#$!%
        // 4. .parse() returns Result which is an enum that has variants 'Ok' and 'Err'
        // Ok(num) => num will return parsed num successfully to guess
        // Err(_) will match any/all errors and call continue to start the loop again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // use match and cmp to compare guess with secret_number and use Ordering
        // to determine if the guess too small, big or correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                // break
                break;
            }
        }
    }
}
