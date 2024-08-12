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

    let res = match req.method.as_str() {
        "GET" => Methods::handle_get(req),
        "POST" => Methods::handle_post(req),
        "PUT" => Methods::handle_put(req),
        "DELETE" => Methods::handle_delete(req),
        &_ => Methods::handle_unsupport(req),
    };

    match stream.write(res.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            println!("FAILED DISPATCHED RESPONSE")
        }
    }
}
