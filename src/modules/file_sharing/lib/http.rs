use super::thread_pool::ThreadPool;
use colored::Colorize;
use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    CatchAll,
}

impl HttpMethod {
    fn from(method: &str) -> HttpMethod {
        match method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => panic!("Cannot convert \"{method}\" to HTTP method."),
        }
    }
}

pub struct HttpRequestTarget {
    path: String,
    query: String,
}

impl HttpRequestTarget {
    fn from(target: &str) -> HttpRequestTarget {
        let (path, query): (&str, &str) = {
            let split: Vec<&str> = target.split('?').collect();
            match split.len() {
                1 => (split[0], ""),
                2 => (split[0], split[1]),
                _ => ("", ""),
            }
        };

        HttpRequestTarget {
            path: path.to_string(),
            query: query.to_string(),
        }
    }
}

pub struct HttpRequest {
    // body
    method: HttpMethod,
    target: HttpRequestTarget,
    headers: HashMap<String, String>,
}

impl HttpRequest {
    fn from(mut stream: &TcpStream) -> Self {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // Parse request line
        let request_line = http_request[0].clone();
        let v: Vec<&str> = request_line.split_whitespace().collect();
        let (method, path) = (HttpMethod::from(v[0]), v[1]);

        let mut headers = HashMap::new();
        for line in http_request {
            if line == "\r\n" {
                break;
            }
            for (i, c) in line.chars().enumerate() {
                if c == ':' {
                    headers.insert(line[0..i].to_lowercase(), line[i + 1..].to_string());
                    break;
                }
            }
        }

        HttpRequest {
            method,
            target: HttpRequestTarget::from(path),
            headers,
        }
    }
}

impl std::fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{method} {path}",
            method = format!("{:?}", self.method).bold(),
            path = self.target.path
        )
    }
}

type HttpHandler = fn(HttpRequest) -> String;

pub struct Server {
    port: usize,
    handlers: Arc<Mutex<Vec<(HttpMethod, String, HttpHandler)>>>,
}

impl Server {
    pub fn new(port: usize) -> Server {
        Server {
            port,
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&mut self, method: HttpMethod, path: &str, handler: HttpHandler) {
        self.handlers
            .lock()
            .unwrap()
            .push((method, path.to_string(), handler));
    }

    pub fn start(&self) {
        let pool = ThreadPool::new(4);
        let host = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(host.clone()).unwrap();
        println!("Listening at {}", host);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let req = HttpRequest::from(&stream);
            let handlers = Arc::clone(&self.handlers);

            pool.execute(move || {
                for (method, path, handler) in handlers.lock().unwrap().iter() {
                    if method == &req.method && path == &req.target.path {
                        let res = handler(req);
                        stream.write_all(res.as_bytes()).unwrap();
                        break;
                    }
                }
            });
        }
    }
}

pub fn handle_static_file(filename: &str) -> String {
    let (status_line, contents) = match fs::read_to_string(format!("public/{filename}")) {
        Ok(contents) => ("HTTP/1.1 200 OK", contents),
        Err(_) => {
            let err404 = fs::read_to_string(format!("public/404.html")).unwrap();
            ("HTTP/1.1 404 NOT FOUND", err404)
        }
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    response
}
