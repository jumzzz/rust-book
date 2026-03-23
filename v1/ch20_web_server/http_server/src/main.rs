use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use http_server::ThreadPool;

const HTML_OK: &str = "html_files/hello.html";
const HTML_SLEEP: &str = "html_files/sleep.html";
const HTML_404: &str = "html_files/404.html";

const REQUEST_OK: &str = "GET / HTTP/1.1";
const REQUEST_SLEEP: &str = "GET /sleep HTTP/1.1";

const STATUS_OK: &str = "HTTP/1.1 200 OK";
const STATUS_404: &str = "HTTP/1.1 404 NOT FOUND";

// Adding /sleep to the index simulates how thread becomes unresponsive
// on Single-Threaded cases
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, html_path) = match &request_line[..] {
        REQUEST_OK => (STATUS_OK, HTML_OK),
        REQUEST_SLEEP => {
            thread::sleep(Duration::from_secs(5));
            (STATUS_OK, HTML_SLEEP)
        }
        _ => (STATUS_404, HTML_404),
    };

    let contents = fs::read_to_string(html_path).unwrap();
    let length = contents.len();

    let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut counter: u64 = 0;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        counter += 1;

        let stream = stream.unwrap();

        println!("stream_count = {}", counter);

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
