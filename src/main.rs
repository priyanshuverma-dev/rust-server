use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use server::methods::Methods;
use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind(("127.0.0.1", 5000)).unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    println!("Connection established!");
                    handle_connection(stream);
                });
            }
            Err(err) => {
                println!("Failed to get stream: {err:?}");
            }
        };
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // let buf_read = Buffer::new([1; 1024]);
    // let request_line: String = buf_read.lines().next().unwrap().unwrap();
    // let request_method = request_line.split_ascii_whitespace().next().unwrap();
    // println!("METHOD: {request_method}!");

    // let http_request: Vec<_> = buf_read
    //     .lines()
    //     .map(|result| result.unwrap())
    //     // .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {http_request:#?}");
    let buf_reader = BufReader::new(&stream);

    let http_request = buf_reader.get_ref();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let status_line = "HTTP/1.1 200 OK";
    let contents = "{'error': 'METHOD NOT SUPPORTED'}";
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}");
    stream.write(response.as_bytes()).unwrap();

    // match request_method {
    //     "GET" => Methods::handle_get(stream),
    //     "POST" => Methods::handle_post(stream),
    //     "PUT" => Methods::handle_put(stream),
    //     "DELETE" => Methods::handle_delete(stream),
    //     &_ => Methods::handle_unsupport(stream),
    // }
}
