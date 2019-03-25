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

    for i in 1..15 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Waits for the thread to finish by calling join().
    // In the log prints, the threads alternate, when the outer loop finishes,
    // the loop on the spawned thread continues, the program blocks until it
    // finishes.
    handle.join().unwrap();
}
