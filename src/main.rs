// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn handle_connection(mut stream: TcpStream) {
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
