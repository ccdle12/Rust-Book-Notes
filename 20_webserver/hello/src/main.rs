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

    // Check if the request is GET.
    // b"" is a byte string.
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // Create the contents of the web page.
    let contents = fs::read_to_string(filename).unwrap();

    // Return with a response.
    let response = format!("{}{}", status_line, contents);

    // Write the response as bytes over the stream.
    stream.write(response.as_bytes()).unwrap();

    // Blocks until all bytes are written.
    stream.flush().unwrap();
}
