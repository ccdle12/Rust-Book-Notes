fn main() {
    // Since we added abort to panice in the Cargo.toml file, rust will not walk back up the stack
    // and release memory. Panice will abort *immediately* and leave it up to the operating system
    // to release memory.
    // panic!("crash and burn")
    

    // This will panice since we are trying to read an index that doesn't exist. AKA *buffer
    // overread*. In C it will return whatever memory is at that array. In Rust it will return a
    // panic.
    // We are running: `RUST_BACKTRACE=1 cargo run`, this will display the whole stack trace.
    let v = vec![1, 2, 3];

    v[99];

}
