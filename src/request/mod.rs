use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub content: String,
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
        let mut req_list = binding.split_inclusive("\n").enumerate();

        let headers: HashMap<String, String> = HashMap::new();

        // metadata METHOD AND PATH
        let (_, req_line_s) = req_list.next().unwrap();
        let mut wd = req_line_s.split_ascii_whitespace();

        let path = wd.next().unwrap();
        let req_method = wd.next().unwrap();

        // let idx = &req_list.position(|c| c.1 == "\r\n").unwrap();

        for (i, c) in &mut req_list {
            // if i < idx {
            //     println!("idx: {}", idx);
            // } else {
            //     if i == 0 {
            //         let mut wd = c.split_ascii_whitespace();
            //         path = wd.next().unwrap();
            //         req_method = wd.next().unwrap();
            //     }
            // }
            println!("id: {}, c: {}", i, c);
        }
        // println!("IDX: {}", idx);
        // let (h, b): (Vec<_>, Vec<_>) = req_list.partition(|(i, _)| i < &mut idx).1;
        let content: String = String::from(" ");
        // println!("{:?}", h);
        Self {
            method: req_method.to_string(),
            path: path.to_string(),
            content,
            headers,
        }
    }
}
