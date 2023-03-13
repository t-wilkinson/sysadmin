pub mod http;
pub mod thread_pool;
use self::http::{handle_static_file, HttpMethod::*, Server};
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    port: usize,
}

impl Cli {
    pub fn run(&self) {
        let mut server = Server::new(self.port);
        server.add(GET, "/", |_req| handle_static_file("hello.html"));
        server.add(CatchAll, "/", |_req| handle_static_file("404.html"));
    }
}

pub fn run() {
    let cli = Cli::parse();
    cli.run();
}
