use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to port 7878");

    // incoming() returns an iterator that gives us streams.
    // A stream represents a connection to the server.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create a buffer on the stack capable of reading 512 bytes.
    let mut buffer = [0; 512];

    // Read the bytes from the stream and write it the buffer.
    stream.read(&mut buffer).unwrap();

    // Borrow the buffer and create a string from_utf8_lossy and print.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // Create the contents of the web page.
    let contents = fs::read_to_string("hello.html").unwrap();

    // Return with a response.
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    // Write the response as bytes over the stream.
    stream.write(response.as_bytes()).unwrap();

    // Blocks until all bytes are written.
    stream.flush().unwrap();
}
