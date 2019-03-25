use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    // mpsc = "multiple producer, single consumer".
    // Fan in pattern - multiple producers a single consumer.
    // tx = transceiver
    // rx = receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Single val sent on the thread.
        // let val = String::from("hi");
        // tx.send(val).unwrap();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // The send function takes ownership of val, sending it down the
            // channel for the receiver to take ownership.
            // println!("val is {}", val);
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv() will block until the message  has been received on the channel.
    // try_recv() will not block.
    // let received = rx.recv().unwrap();
    // println!("Got {}", received);

    for received in rx {
        println!("Got: {}", received);
    }
}
