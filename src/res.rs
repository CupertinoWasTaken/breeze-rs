use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::status::StatusCode;
use crate::VERSION;

#[derive(Debug, Default)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        let mut response = Self::default();
        response.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        response.headers.insert("Content-Length".to_string(), "0".to_string());

        response
    }

    pub fn set_body(&mut self, new_body: impl Display) {
        let body = new_body.to_string();
        let length = body.as_bytes().len();

        self.body = Some(body);
        self.headers.remove("Content-Length");
        self.headers.insert("Content-Length".to_string(), length.to_string());
    }

    pub fn stringify_headers(&self) -> String {
        let mut output = String::new();

        for (key, value) in &self.headers {
            let header = key.to_owned() + ": " + value + "\r\n";
            output.push_str(&header)
        }

        output
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let body = self.body.to_owned();

        output += VERSION;
        output.push(' ');
        output += self.status.to_string().as_str();
        output.push_str("\r\n");
        output += &self.stringify_headers();
        output.push_str("\r\n");
        output += &body.unwrap_or_default();

        write!(f, "{output}")
    }
}