use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a mutex with a value.
    let m = Mutex::new(5);

    {
        // Acquire the lock for the mutex, this will block until the lock is
        // acquired. If the lock as already acquired, the lock() will panic.
        let mut num = m.lock().unwrap();

        // Lock() returns a smart pointer, specifically a smart pointer named
        // MutexGuard. This implements Drop, so the mutex guard will be dropped
        // if out of scope. Also implements Deref.
        *num = 6;
    }

    println!("m = {:?}", m);

    // Example: Fails because of counter has already been moved into the closure
    // after the first iteration.
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter.lock().unwrap());

    // We can fix the above by allowing multiple owners fo the Mutex using
    // reference counting, Rc<>.
    // Rc<> cannot be sent on threads safetly.
    // Rc<> increments the reference count when clone() is called and then drops
    // it when it is dropped from scope.
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter.lock().unwrap());

    // We can use reference counting atomically by using an Atomic Reference
    // Counting Arc<>.
    // NOTE: Arc<> has runtime performance costs.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
