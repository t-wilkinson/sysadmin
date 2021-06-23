pub mod auths {
    pub fn fetch() {
        // TODO
    }
}

pub mod journal {
    use chrono::prelude::{DateTime, Utc};
    use regex::Regex;
    use std::collections::HashMap;
    use std::process::{Command, Output};
    use std::str;
    use std::time::{Duration, SystemTime};

    pub fn fetch(services: Vec<&str>) -> String {
        let output = read(services);
        let lines = std::str::from_utf8(&output.stdout).unwrap().lines();
        let logs = to_logs(lines);
        let services = to_services(logs);
        let json = format!("{{ {} }}", services.join(","));
        json
    }
    fn read(services: Vec<&str>) -> Output {
        let one_day = Duration::new(60 * 60 * 24, 0);
        let time: DateTime<Utc> = (SystemTime::now() - one_day).into();
        let time = time.format("%Y-%m-%d %H:%M:%S").to_string();

        let args = vec!["-r", "-S", &time, "-o", "json", "-u"].into_iter();
        let services = services.into_iter().intersperse("-u");

        let output = Command::new("journalctl")
            .args(args.chain(services))
            .output()
            .expect("Unable to run journalctl");
        if !output.status.success() {
            panic!("Could not fetch logs");
        }
        output
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
        logs.iter()
            .map(|(service, log)| {
                let log = format!("[{}]", log.join(","));
                format!("{:?}: {}", service, log)
            })
            .collect()
    }
}

pub mod sar {
    use serde_json::{map::Map, value::Value};
    use std::process::{Command, Output};

    pub fn fetch() -> String {
        let output = read();
        let reports = std::str::from_utf8(&output.stdout).unwrap().split("\n\n");
        let mut json: Map<String, Value> = Map::new();

        for report in reports {
            let mut key = String::from("");
            let mut results = Vec::new();

            for (i, line) in report.lines().enumerate() {
                let mut line = line.split_whitespace();
                line.next();
                let line: Value = line.collect();

                if i == 0 {
                    key = line[0].as_str().unwrap().to_string();
                }

                results.push(line);
            }

            json.insert(key, serde_json::json!(results));
        }

        let json = serde_json::to_string(&json).unwrap();
        json
    }

    fn read() -> Output {
        let output = Command::new("sar")
            .args(&["-d", "-u", "-w", "-n", "DEV", "1", "2"])
            .output()
            .expect("Unable to run sar");
        if !output.status.success() {
            panic!("Could not fetch logs");
        }
        output
    }
}
