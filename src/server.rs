use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::collections::HashMap;

use std::fs;

pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

#[derive(Debug, Clone)]
enum Method {
    Get,
    Post,
}

#[derive(Debug, Clone)]
struct Req {
    path: String,
    method: Method,
    params: HashMap<String, String>
}

impl Req {
    fn new(path: String, method: Method, params: HashMap<String, String>) -> Req {
        Req {path, method, params}
    }
}

fn read_request(req: String) -> Req {
    let mut params = HashMap::new();
    let index_of_newline = req.find('\n').unwrap_or(0);
    let header = req[0..index_of_newline].to_string();
    let req_params = req[(index_of_newline+2)..req.len()-1].to_string();

    let header: Vec<&str> = header.split(" ").collect();
    let method = match header[0] {
        "GET" => Method::Get,
        "POST" => Method::Post,
        method => panic!("Malformed method {:?}", method),
    };
    let path = header[1].to_string();
    // let version = header[2].to_string();

    for line in req_params.split("\n") {
        if line == "\r" {
            // Final line break.
            break
        }
        let index_of_colon = line.find(':').unwrap_or(0);
        let head = line[0..index_of_colon].to_string();
        let tail = line[(index_of_colon+2)..line.len()-1].to_string();
        params.insert(head, tail);
    }
    Req::new(path, method, params)
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer[..]);
    let request = read_request(request_str.to_string());
    println!("Request: {:#?}", request);

    let contents = if request.path == "/" {
        fs::read_to_string("files/hello.html").unwrap()
    } else {
        fs::read_to_string("files/bye.html").unwrap()
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
