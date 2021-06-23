use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    stream: TcpStream,
    headers: HashMap<String, String>,
    body: String,
    http_status: u16,
    http_version: String,
    http_reason: String,
}

// macro_rules! chain {
//     ($field_name:ident, $fn_name:ident, $type:ty, $exp:expr) => {
//         pub fn $fn_name(&mut self, $field_name: $type) -> &mut Self {
//             self.$field_name = $exp;
//             self
//         }
//     };
// }

impl Response {
    pub fn new(stream: TcpStream) -> Self {
        Response {
            stream,
            headers: HashMap::new(),
            body: String::new(),
            http_status: 0,
            http_version: String::from("HTTP/1.1"),
            http_reason: String::new(),
        }
    }

    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn status(&mut self, status: u16) -> &mut Self {
        self.http_status = status;
        self
    }

    // chain! {body, body, &str, body.to_string() }
    pub fn body(&mut self, body: &str) -> &mut Self {
        self.body = body.to_string();
        if self.http_status == 0 {
            self.http_status = 200
        }
        self
    }

    fn message(&self) -> String {
        const CRLN: &str = "\r\n";

        let status_line = format!(
            "{} {} {}",
            self.http_version, self.http_status, self.http_reason
        );
        let headers = self
            .headers
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join(CRLN);

        // body should have two \r\n
        let message = format!(
            "{}{CRLN}{}{CRLN}{CRLN}{}",
            status_line,
            headers,
            self.body,
            CRLN = CRLN
        );

        message
    }

    fn write(&mut self) {
        let message = self.message();
        let message = message.as_bytes();
        self.stream.write(message).unwrap();
    }
}

impl Drop for Response {
    fn drop(&mut self) {
        self.write();
    }
}
