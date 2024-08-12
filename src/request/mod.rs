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
        let mut req_list: Vec<&str> = binding.split_inclusive("\n").collect();

        let mut headers: HashMap<String, String> = HashMap::new();
        // metadata METHOD AND PATH
        let req_line_s = req_list[0];
        let mut wd = req_line_s.split_ascii_whitespace();
        // remember to get this first
        let req_method = wd.next().unwrap();
        // this second it can mess. I took me 2 hours!
        let path = wd.next().unwrap();

        let idx = req_list.iter().position(|&r| r == "\r\n").unwrap();

        // at this point i am a genius. I love rust!
        // fixed this shit and extracted the content from this.
        let b = &mut req_list.split_off(idx);
        b.remove(0); // removes spaces
        let content: String = String::from(b.join(""));

        // now let's fix the header
        req_list.remove(0); // remove meta line
                            // println!("H: {:#?} B: {:#?}", headers, b);
        for head in req_list {
            let (name, value) = head.split_once(": ").unwrap();
            headers.insert(name.to_string(), value.to_string().replace("\r\n", ""));
        }

        Self {
            method: req_method.to_string(),
            path: path.to_string(),
            content,
            headers,
        }
    }
}
