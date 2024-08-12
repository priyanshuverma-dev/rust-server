use crate::{constants, request::Request};

pub struct Methods;

impl Methods {
    pub fn handle_get(req: Request) -> &'static str {
        println!("fn: {} path: {}", req.method, req.path);
        constants::DEFAULT_RESPONSE
    }
    pub fn handle_post(req: Request) -> &'static str {
        println!("fn: {} path: {}", req.method, req.path);
        constants::DEFAULT_RESPONSE
    }
    pub fn handle_put(req: Request) -> &'static str {
        println!("fn: {} path: {}", req.method, req.path);
        constants::DEFAULT_RESPONSE
    }
    pub fn handle_delete(req: Request) -> &'static str {
        println!("fn: {} path: {}", req.method, req.path);
        constants::DEFAULT_RESPONSE
    }
    pub fn handle_unsupport(req: Request) -> &'static str {
        println!("fn: {} path: {}", req.method, req.path);
        constants::DEFAULT_RESPONSE
    }
}
