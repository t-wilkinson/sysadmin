use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
}

impl Request {}
