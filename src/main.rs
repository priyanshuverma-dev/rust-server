use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind(("127.0.0.1", 80)).unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_read = BufReader::new(&mut stream);
    let request_line: String = buf_read.lines().next().unwrap().unwrap();
    let request_method = request_line.split_ascii_whitespace().next().unwrap();
    match request_method {
        "GET" => {
            println!("Method is GET.")
        }
        "POST" => {
            println!("Method is POST.")
        }
        "PUT" => {
            println!("Method is PUT.")
        }
        "DELETE" => {
            println!("Method is DELETE.")
        }
        &_ => {
            println!("Method not supported for now.")
        }
    }
    let status_line = "HTTP/1.1 200 OK";
    let contents = "{'hello': 'World'}";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
}
