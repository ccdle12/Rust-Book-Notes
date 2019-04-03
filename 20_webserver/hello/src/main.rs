use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to port 7878");

    // incoming() returns an iterator that gives us streams.
    // A stream represents a connection to the server.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
