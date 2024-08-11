use crate::constants;

pub struct Methods;

// looks good


impl Methods {
    pub fn handle_get() -> &'static str {
        constants::DEFAULT_RESPONSE
    }
    pub fn handle_post() -> &'static str {
        constants::DEFAULT_RESPONSE
    }
    pub fn handle_put() -> &'static str {
        constants::DEFAULT_RESPONSE
    }
    pub fn handle_delete() -> &'static str {
        constants::DEFAULT_RESPONSE
    }
    pub fn handle_unsupport() -> &'static str {
        constants::DEFAULT_RESPONSE
    }
}
