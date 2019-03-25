use std::thread;
use std::time::Duration;

fn main() {
    // Rust uses 1:1 threads.
    // Assigning the spawned thread to handle and calling join() later on will
    // wait for the thread to finish.
    let handle = thread::spawn(|| {
        for i in 1..20 {
            println!("hi number {} from the spawned thread!", i);
            // Sleep stops execution for some time, allowing the other thread
            // to continue.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Move the join call, blocks the outer for loop from running until the
    // handle is finished.
    handle.join().unwrap();

    for i in 1..15 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Waits for the thread to finish by calling join().
    // In the log prints, the threads alternate, when the outer loop finishes,
    // the loop on the spawned thread continues, the program blocks until it
    // finishes.
    // handle.join().unwrap();

    // Using move to borrow a variable in a thread. Since the thread closure
    // has no knowledge of it's outside varibales we need to use "move", to
    // borrow the outside variable in the closure.
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
