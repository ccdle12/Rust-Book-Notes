use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let sim_user_val = 10;
    let sim_rand_num = 7;

    gen_workout(sim_user_val, sim_rand_num);
}

// fn sim_expensive_calc(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// Create a struct that will have a closure and result as its fields.
// We use the trait Fn. All closures implement Fn, FnMut or FnOnce.
struct Cacher<T>
where
    // T is the generic type that is bound to a closure by using trait Fn.
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // Takes a generic parameter T that implements the Fn trait. It returns an instance of the
    // cacher holding the calculation (the closure) and a value set to None since we haven't
    // executed the closure.
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    // When calling value(), we only execute the closure if the closure has been executed and has
    // created a value.
    fn value(&mut self, arg: u32) -> u32 {
        // Check that the value HashMap contains keys.
        match self.value.contains_key(&arg) {
            // Dereferencing the borrow if true.
            true => *self.value.get(&arg).unwrap(),
            false => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn gen_workout(intensity: u32, rand_num: u32) {
    // let expensive_result = sim_expensive_calc(intensity);

    // Closure, for simulating the expensive_closure
    // Annotation in a closure is optional.
    // let expensive_closure = |num: u32| -> u32 {
    // println!("calculating slowly...");
    // thread::sleep(Duration::from_secs(2));
    // num
    // };

    // Creating the expensive_result using the cacher struct.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // By using the cacher, we can call the expensive funciton only ONCE, since it will just
    // retrieve the value of the expensive result.
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result.value(intensity));
        println!("Next do {} situps", expensive_result.value(intensity));
    } else {
        if rand_num == 3 {
            println!("Take a break today!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // The below fails because the closure has already inferred the type to be a string.
    // let n = example_closure(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn env_closure() {
        // Closures can capture variables in it's environment.
        // Capturing variables from environments is done in 3 ways (using the Fn trait):
        // FnOnce:
        //  Consumes the variable it captures from its enclosing scope. The closure takes ownership
        //  of the environment variable.
        // FnMut:
        //  Borrows the environment variable mutably.
        // Fn:
        //  Borrows the environment variable imutably
        let x = 4;
        // At the moment, the closure borrows x.
        let equal_to_x = |z| z == x;

        let y = 4;

        assert!(equal_to_x(y));

        // test that we can reuse x.
        assert_eq!(x, 4);

        // Use the "move" keyword to move ownership to a closure.
        let equal_to_x = move |z: u32| z == x;
    }
}
