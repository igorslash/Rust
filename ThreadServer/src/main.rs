mod thread_pool;

use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use ::rayon::prelude::*;
use rayon::ThreadPoolBuilder;

const INDEX_HTML: &str = "index.html";
const NOT_FOUND_HTML: &str = "404.html";
const STATUS_OK: &str = "HTTP/1.1 200 OK";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").unwrap();
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_read = BufReader::new(&stream);
    let requests = buf_read.lines().next().unwrap().unwrap();
    let (status_line, filename) = if requests == "GET / HTTP/1.1" {
        thread::sleep(std::time::Duration::from_secs(5));
        (STATUS_OK, INDEX_HTML)
    } else {
        (STATUS_NOT_FOUND, NOT_FOUND_HTML)
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",
        status_line = status_line,
        length = length,
        contents = contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
