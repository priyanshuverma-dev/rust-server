use std::{
    io::{prelude::*, BufReader, Write},
    net::TcpStream,
};

pub struct Methods;

impl Methods {
    pub fn handle_get(mut stream: TcpStream) {
        println!("METHOD: INSIDE!");
        let buf_reader = BufReader::new(&stream);

        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {http_request:#?}");

        let status_line = "HTTP/1.1 200 OK";
        let contents = "{'error': 'METHOD NOT SUPPORTED'}";
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    pub fn handle_post(mut stream: TcpStream) {
        println!("METHOD: INSIDE!");
        // let buf_reader = BufReader::new(&mut stream);
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        // let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).collect();

        let status_line = "HTTP/1.1 200 OK";
        let contents = "{'error': 'METHOD NOT SUPPORTED'}";
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    pub fn handle_put(mut stream: TcpStream) {
        let status_line = "HTTP/1.1 200 OK";
        let contents = "{'error': 'METHOD NOT SUPPORTED'}";
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    pub fn handle_delete(mut stream: TcpStream) {
        let status_line = "HTTP/1.1 200 OK";
        let contents = "{'error': 'METHOD NOT SUPPORTED'}";
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    pub fn handle_unsupport(mut stream: TcpStream) {
        let status_line = "HTTP/1.1 501 OK";
        let contents = "{'error': 'METHOD NOT SUPPORTED'}";
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
