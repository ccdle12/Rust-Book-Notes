use std::sync::Mutex;

fn main() {
    // Create a mutex with a value.
    let m = Mutex::new(5);

    {
        // Acquire the lock for the mutex, this will block until the lock is
        // acquired. If the lock as already acquired, the lock() will panic.
        let mut num = m.lock().unwrap();

        // Lock() returns a smart pointer, specifically a smart pointer named
        // MutexGuard. This implements Drop, so the mutex guard will be dropped
        // if out of scope.
        *num = 6;
    }

    println!("m = {:?}", m);
}
