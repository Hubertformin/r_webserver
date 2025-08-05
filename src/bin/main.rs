extern crate webserver;
use webserver::ThreadPool;

use std::fs::File;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running at http://127.0.0.1:7878");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();


        pool.execute(|| {
            handle_request(stream);
        });
    }
    
    println!("Shutting down.");
}

fn handle_request(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let mut request_lines = reader.lines();
    let mut request_str = String::new();

    // read the lines from the request body
    while let Some(line) = request_lines.next() {
        let line = line.unwrap();
        request_str.push_str(&line);
        request_str.push_str("\r\n");

        if line.is_empty() {
            break;
        }
    }

    let get = "GET / HTTP/1.1\r\n";
    let sleep = "GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if request_str.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if request_str.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let length = contents.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}