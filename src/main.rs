use std::env::args;
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use http_server_starter_rust::ThreadPool;

const THREAD_POOL_SIZE: usize = 4;

fn handle_connection(mut stream: TcpStream) {
    let dir_name = args().nth(2).unwrap_or(String::from(""));

    let buf_reader = BufReader::new(&mut stream);
    let mut request = buf_reader.lines();
    let request_line = request.next().unwrap().unwrap();
    let chunks: Vec<&str> = request_line.split_whitespace().collect();
    let path = chunks[1];

    if path == "/" {
        stream
            .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .expect("Unable to write to stream");
    } else if path == "/user-agent" {
        let user_agent_line = request
            .map(|l| l.unwrap())
            .filter(|l| l.starts_with("User-Agent"))
            .next()
            .unwrap();
        let user_agent = user_agent_line.strip_prefix("User-Agent:").unwrap().trim();
        let out = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", user_agent.as_bytes().len(), user_agent);
        stream
            .write(out.as_bytes())
            .expect("Unable to write to stream");
    } else if path.starts_with("/echo/") {
        let res_text = path.strip_prefix("/echo/").expect("expected string as input");
        let out = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", res_text.as_bytes().len(), res_text);
        stream
            .write(out.as_bytes())
            .expect("Unable to write to stream");
    } else if path.starts_with("/files/") {
        let file_name = path.strip_prefix("/files/").expect("missing filename in request");
        if file_name.contains('/') {
            let out = format!("HTTP/1.1 400 Bad Request\r\n\r\n");
            stream
                .write(out.as_bytes())
                .expect("Unable to write to stream");
        } else {
            let full_path = dir_name + file_name;
            let file_path = Path::new(full_path.as_str());
            if file_path.exists() {
                let file_contents = fs::read_to_string(&full_path).unwrap();
                let length = file_contents.as_bytes().len();
                let out = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\n{}\r\n\r\n", length, file_contents);
                stream
                    .write(out.as_bytes())
                    .expect("Unable to write to stream");
            } else {
                let out = format!("HTTP/1.1 404 Not Found\r\n\r\n");
                stream
                    .write(out.as_bytes())
                    .expect("Unable to write to stream");
            }
        }
    } else {
        stream
            .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .expect("Unable to write to stream");
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(THREAD_POOL_SIZE);

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                pool.execute(|| {
                    handle_connection(_stream);
                });
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    // ThreadPool goes out of scope here, so drop is called for it
}
