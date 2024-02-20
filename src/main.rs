use std::env::args;
use std::{fs, io};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use http_server_starter_rust::ThreadPool;

const THREAD_POOL_SIZE: usize = 4;

fn handle_connection(mut stream: TcpStream) {
    let dir_name = args().nth(2).unwrap_or(String::from(""));

    let mut buf_reader = BufReader::new(& mut stream);
    let received: Vec<u8> = buf_reader.fill_buf().expect("Could not read request into buffer").to_vec();

    let req_string = String::from_utf8(received.clone())
        .map(|msg| {
            buf_reader.consume(received.len());
            msg
        })
        .map_err(|_| {
            return io::Error::new(
                io::ErrorKind::InvalidData,
                "Could not parse received string as utf8",
            );
        })
        .unwrap();

    let mut head = Vec::new();

    for line in req_string.split("\n") {
        println!("{}", line);
        if line == "\n" {
            break;
        }
        head.push(line);
    }

    let (start_line, headers_line) = head.split_at(1);

    println!("{:?}-{:?}", start_line, headers_line);

    let binding = start_line.concat().to_string();
    let req_start: Vec<&str> = binding.split_whitespace().collect();
    let req_method = req_start[0];
    let req_path = req_start[1];
    let http_ver = req_start[2];

    let req_body_index = headers_line.iter().position(|&l| l == "\r").unwrap() + 1;

    println!("req body index {}", req_body_index);

    println!("{} {} {}", req_method, req_path, http_ver);

    let mut headers = HashMap::new();
    for header_line in headers_line {
        let mut parts = header_line.split(":");
        if parts.clone().count() == 2 {
            headers.insert(
                parts.next().map(str::to_string).unwrap(),
                parts.next().map(str::to_string).unwrap().trim().to_string()
            );
        }
    }

    println!("{:?}", headers);

    // let mut request = buf_reader.lines();
    // let request_line = request.next().unwrap().unwrap();
    // let chunks: Vec<&str> = request_line.split_whitespace().collect();
    // let request_verb = chunks[0];
    // let path = chunks[1];

    // TODO: refactor this to use pattern matching, instead, as it's much more Rustacean
    if req_path == "/" {
        stream
            .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .expect("Unable to write to stream");
    } else if req_path == "/user-agent" {
        // let user_agent_line = request
        //     .map(|l| l.unwrap())
        //     .filter(|l| l.starts_with("User-Agent"))
        //     .next()
        //     .unwrap();
        // let user_agent = user_agent_line.strip_prefix("User-Agent:").unwrap().trim();
        let user_agent = headers.get("User-Agent").unwrap();
        let out = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", user_agent.as_bytes().len(), user_agent);
        stream
            .write(out.as_bytes())
            .expect("Unable to write to stream");
    } else if req_path.starts_with("/echo/") {
        let res_text = req_path.strip_prefix("/echo/").expect("expected string as input");
        let out = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", res_text.as_bytes().len(), res_text);
        stream
            .write(out.as_bytes())
            .expect("Unable to write to stream");
    } else if req_path.starts_with("/files/") {
        if req_method == "GET" {
            let file_name = req_path.strip_prefix("/files/").expect("missing filename in request");
            if file_name.contains('/') {
                let out = "HTTP/1.1 400 Bad Request\r\n\r\n".to_string();
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
                    let out = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
                    stream
                        .write(out.as_bytes())
                        .expect("Unable to write to stream");
                }
            }
        } else if req_method == "POST" {
            let file_name = req_path.strip_prefix("/files/").expect("missing filename in request");
            let full_path = dir_name + file_name;
            let file_path = Path::new(full_path.as_str());

            let file_contents = headers_line.get(req_body_index).unwrap();

            fs::write(file_path, file_contents).expect("Unable to write contents to file");
            let out = "HTTP/1.1 201 Created\r\n\r\n".to_string();
            stream
                .write(out.as_bytes())
                .expect("Unable to write to stream");

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
            Ok(_stream) => {
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

    // ThreadPool goes out of scope here, so drop is called on it
}
