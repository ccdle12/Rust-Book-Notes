use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to port 7878");
    // Create a thread pool with 4 threads.
    let pool = ThreadPool::new(4);

    // incoming() returns an iterator that gives us streams.
    // A stream represents a connection to the server.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // We can create a thread for each request to handle multiple requests
        // at once. This has the drawback though, of DoS attacks that can put
        // a lot of strain on our server.
        // thread::spawn(|| {
        // handle_connection(stream);
        // });
        pool.execute(|| {
            handle_connection(stream);
        });
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

    // This is currently a single threaded server, we will simulate this by
    // adding a sleep, to the /sleep. This will wait 5 seconds, and if we try
    // to access the webserver using the default /. We will have to wait until
    // the /sleep endpoint has been served.
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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
