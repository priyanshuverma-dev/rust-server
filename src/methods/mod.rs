use crate::{constants, request::Request};

pub struct Methods;

impl Methods {
    pub fn handle_get(req: Request) -> String {
        println!("{:#?}", req);
        constants::DEFAULT_RESPONSE.to_string()
    }
    pub fn handle_post(req: Request) -> String {
        println!("{:#?}", req);
        constants::DEFAULT_RESPONSE.to_string()
    }
    pub fn handle_put(req: Request) -> String {
        println!("{:#?}", req);
        constants::DEFAULT_RESPONSE.to_string()
    }
    pub fn handle_delete(req: Request) -> String {
        println!("{:#?}", req);
        constants::DEFAULT_RESPONSE.to_string()
    }
    pub fn handle_error(error: &str) -> String {
        let payload = format!("{{'message': '{}'}}", error);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
            payload.len(),
            payload
        );

        response
    }
}
