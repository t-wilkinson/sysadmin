use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::str;
use std::vec::Vec;

#[derive(Debug, Clone)]
enum ServerError {
    ReadFile,
}

use std::fmt;

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::ReadFile => write!(f, "Unable to read file"),
        }
    }
}

impl From<std::io::Error> for ServerError {
    fn from(_error: std::io::Error) -> Self {
        ServerError::ReadFile
    }
}

impl From<std::str::Utf8Error> for ServerError {
    fn from(_error: std::str::Utf8Error) -> Self {
        ServerError::ReadFile
    }
}

type Route = String;

// type Middleware = fn(&mut Request, &mut Response);

pub struct Server {
    port: u16,
    host: String,
    routes: HashMap<Route, Handler>,
    // middleware: Vec<Middleware>,
}

type Handler = fn(Request, &mut Response);

impl Server {
    pub fn new(port: u16) -> Self {
        Server {
            port,
            host: String::from("127.0.0.1"),
            routes: HashMap::new(),
            // middleware: Vec::new(),
        }
    }

    pub fn route(&mut self, route: &str, handler: Handler) {
        self.routes.insert(route.to_string(), handler);
    }

    //     pub fn add(&mut self, middleware: Middleware) {
    //         self.middleware.push(middleware);
    //     }

    pub fn listen(&self) {
        let address = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&address).unwrap();
        let mime_types: HashMap<&str, &str> = vec![
            ("html", "text/html"),
            ("css", "text/css"),
            ("js", "text/javascript"),
        ]
        .into_iter()
        .collect();

        println!("Serving on {}", &address);

        // accept connections and process them serially
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let req = self.to_request(&mut stream);
            let mut res = Response::new(stream);

            if let Some(handler) = self.routes.get(&req.path) {
                handler(req, &mut res);
            } else {
                if !Path::new(&req.path).is_absolute() {
                    panic!("Cannot handle relative paths")
                }

                let mut read_file = || -> Result<(), ServerError> {
                    let mut path = format!("./web{}", req.path);
                    if path.ends_with('/') {
                        path = path + "index.html";
                    }
                    let path = Path::new(&path);
                    let path = fs::canonicalize(path)?;

                    let ext = path.extension().and_then(|ext| ext.to_str());
                    if let Some(ext) = ext {
                        res.header("Content-Type", mime_types.get(ext).unwrap());
                    }

                    let file = fs::read(path)?;
                    res.body(str::from_utf8(&file)?);
                    Ok(())
                };

                if let Err(_err) = read_file() {
                    println!("Unable to read file: {}", req.path);
                }
            }
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
