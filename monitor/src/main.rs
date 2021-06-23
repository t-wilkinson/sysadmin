/* TODO
 * develop frontend to make requests to backend
 * write json parser
 * specify wanted services
 * improve api
 * reload html on file change
 * [handle signint](https://rust-cli.github.io/book/in-depth/signals.html)
 * compress data before sending
 * use https://en.wikipedia.org/wiki/WebSocket
 * implement logger for binary
 */

#![feature(iter_intersperse)]
#![allow(dead_code)]
mod bucket;
mod request;
mod response;
mod server;
use crate::bucket::{journal, sar};
use crate::request::Request;
use crate::response::Response;
use crate::server::Server;

fn handle_sar(_req: Request, res: &mut Response) {
    let json = sar::fetch();
    res.header("Content-Type", "application/json").body(&json);
}

fn handle_journal(_req: Request, res: &mut Response) {
    let services = vec!["docker"];
    let json = journal::fetch(services);
    res.header("Content-Type", "application/json").body(&json);
}

fn main() {
    let mut server = Server::new(8888);
    server.route("/sar", handle_sar);
    server.route("/journal", handle_journal);
    server.listen();
}
