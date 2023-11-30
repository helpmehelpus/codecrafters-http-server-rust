// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let chunks: Vec<&str> = request_line.split_whitespace().collect();
    let path = chunks[1];

    if path == "/" {
        stream
            .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .expect("Unable to write to stream");
    } else if path.starts_with("/echo/") {
        let res_text = path.strip_prefix("/echo/").expect("expected string as input");
        let out = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", res_text.as_bytes().len(), res_text);
        stream
            .write(out.as_bytes())
            .expect("Unable to write to stream");
    } else {
        stream
            .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .expect("Unable to write to stream");
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(_stream);
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
