use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
}
const MESSAGE_SIZE: usize = 1024;

impl Request {
    pub fn new(mut stream: &TcpStream) -> Self {
        // full request recieved
        let mut received: Vec<u8> = vec![];

        // request pieces in bytes vec
        let mut rx_bytes = [0u8; MESSAGE_SIZE];

        loop {
            // Read from the current data in the TcpStream
            let bytes_read = stream.read(&mut rx_bytes).unwrap();

            // However many bytes we read, extend the `received` string bytes
            received.extend_from_slice(&rx_bytes[..bytes_read]);

            // If we didn't fill the array
            // stop reading because there's no more data (we hope!)
            if bytes_read < MESSAGE_SIZE {
                break;
            }
        }
        let binding = String::from_utf8(received).unwrap();
        println!("{binding}");
        let method = binding.split_ascii_whitespace().next().unwrap();
        let path = binding.split_ascii_whitespace().nth(1).unwrap();

        Self {
            method: method.to_string(),
            path: path.to_string(),
        }
    }
}
