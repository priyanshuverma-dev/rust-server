use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use server::ThreadPool;
use server::{methods::Methods, request::Request};

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
    let req = Request::new(&stream);

    println!("REQUEST: {req:?}");

    let res = match req.method.as_str() {
        "GET" => Methods::handle_get(),
        "POST" => Methods::handle_post(),
        "PUT" => Methods::handle_put(),
        "DELETE" => Methods::handle_delete(),
        &_ => Methods::handle_unsupport(),
    };

    match stream.write(res.as_bytes()) {
        Ok(_) => {
            println!("DISPATCHED RESPONSE")
        }
        Err(_) => {
            println!("FAILED DISPATCHED RESPONSE")
        }
    }
}
