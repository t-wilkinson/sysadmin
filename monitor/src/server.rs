use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::vec::Vec;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
}

impl Request {}

#[derive(Debug)]
pub struct Response {
    stream: TcpStream,
    headers: HashMap<String, String>,
    body: String,
    http_status: u16,
    http_version: String,
    http_reason: String,
}

impl Response {
    fn new(stream: TcpStream) -> Self {
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
    pub fn body(&mut self, body: &str) -> &mut Self {
        self.body = body.to_string();
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
            .clone()
            .into_iter()
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
}

#[derive(Debug)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server {
            port,
            host: String::from("127.0.0.1"),
        }
    }

    pub fn listen(&self, handler: fn(Request, &mut Response)) {
        let address = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&address).unwrap();
        println!("Serving on {}", &address);

        // accept connections and process them serially
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let request = self.to_request(&mut stream);
            let mut res = Response::new(stream);

            handler(request, &mut res);

            let message = res.message();
            let message = message.as_bytes();
            res.stream.write(message).unwrap();
        }
    }

    fn to_request(&self, stream: &mut TcpStream) -> Request {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let mut method = "";
        let mut path = "";
        let mut headers = HashMap::new();
        let http_req = str::from_utf8(&buffer).unwrap().trim_end();

        for (i, line) in http_req.lines().enumerate() {
            if line == "" {
                break;
            } else if i == 0 {
                let vals: Vec<&str> = line.split(' ').collect();
                method = vals[0];
                path = vals[1];
            } else {
                if let Some((header, value)) = line.split_once(": ") {
                    headers.insert(String::from(header), String::from(value));
                };
            }
        }

        if method.is_empty() || path.is_empty() {
            panic!("Couldn't parse http request");
        }

        let request = Request {
            method: method.to_string(),
            path: path.to_string(),
            headers,
        };

        request
    }
}
