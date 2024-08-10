use std::{io::BufReader, net::TcpStream};

pub struct Helper {
    buffer: BufReader<TcpStream>,
}

impl Helper {
    pub fn get_method(&self) {}
}
