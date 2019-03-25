use std::sync::mpsc;
use std::thread;

fn main() {
    println!("Hello, world!");
    // mpsc = "multiple producer, single consumer".
    // Fan in pattern - multiple producers a single consumer.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv() will block until the message  has been received on the channel.
    // try_recv() will not block.
    let received = rx.recv().unwrap();
    println!("Got {}", received);
}
