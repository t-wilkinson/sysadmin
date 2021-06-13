/* TODO
 * learn rust
 *  - lifecycles
 *  - str vs String
 *  - iter
 *  - collections
 *  - str
 *  - macros
 * improve api
 * specify wanted services
 * develop frontend to make requests to backend
 * include more metrics
 *  [handle signint](https://rust-cli.github.io/book/in-depth/signals.html)
 * implement logger for binary
 */

#![allow(dead_code)]
mod server;
use crate::server::{Request, Response, Server};
use chrono::prelude::{DateTime, Utc};
use regex::Regex;
use std::collections::HashMap;
use std::process::{Command, Output};
use std::str;
use std::time::{Duration, SystemTime};

fn fetch_journal() -> Output {
    let one_day = Duration::new(60 * 60 * 24, 0);
    let time: DateTime<Utc> = (SystemTime::now() - one_day).into();
    let time = time.format("%Y-%m-%d %H:%M:%S").to_string();
    Command::new("journalctl")
        .args(&["-r", "-u", "docker", "-S", &time, "-o", "json"])
        .output()
        .expect("Unable to run journalctl")
}

type Logs = HashMap<String, Vec<String>>;

fn to_logs(lines: str::Lines) -> Logs {
    let mut logs = HashMap::new();
    let re = Regex::new(r#""SYSLOG_IDENTIFIER":"(\w*)""#).unwrap();
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let service = caps.get(1).unwrap().as_str().to_string();
        let log = logs.entry(service).or_insert(Vec::new());
        log.push(line.to_string());
    }
    logs
}

fn to_services(logs: Logs) -> Vec<String> {
    logs.into_iter()
        .map(|(service, log)| {
            let log = format!("[{}]", log.join(","));
            format!("{:?}: {}", service, log)
        })
        .collect()
}

fn handler(_req: Request, res: &mut Response) {
    let output = fetch_journal();
    if !output.status.success() {
        panic!("Could not fetch logs");
    }

    let lines = str::from_utf8(&output.stdout).unwrap().lines();
    let logs = to_logs(lines);
    let services = to_services(logs);
    let json = format!("{{ {} }}", services.join(","));
    res.header("Content-Type", "application/json")
        .status(200)
        .body(&json);
}

fn main() {
    let server = Server::new(8888);
    server.listen(handler);
}
