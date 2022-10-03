/// Normalizes file paths
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, io};

lazy_static! {
    static ref RE_SIMPLE_CHARS: Regex = Regex::new(r"[^\w\s-]").unwrap();
    static ref RE_DUPLICATE_CHARS: Regex = Regex::new(r"[-\s_]+").unwrap();
}

fn normalize_string(string: &str) -> String {
    let string = string.to_lowercase();
    let string = string.trim();
    let string = RE_SIMPLE_CHARS.replace_all(string, "").to_string();
    let string = RE_DUPLICATE_CHARS.replace_all(&string, "-").to_string();
    string
}

fn normalize_files(path: &str) -> Result<(), io::Error> {
    let md = fs::metadata(path).unwrap();

    if md.is_dir() {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            let path = path.to_str().unwrap();
            fs::rename(path, normalize_string(path)).unwrap();
        }
    } else if md.is_file() {
        fs::rename(path, normalize_string(path)).unwrap();
    }
    Ok(())
}

fn normalize_stdin() {
    for line in io::stdin().lines() {
        println!("{}", normalize_string(&line.unwrap()));
    }
}

#[derive(Parser)]
pub struct Cli {
    path: Option<String>,
}

impl Cli {
    pub fn run(&self) {
        if let Some(path) = &self.path {
            normalize_files(path).unwrap();
        } else {
            normalize_stdin();
        }
    }
}

pub fn run() {
    let cli = Cli::parse();
    cli.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_strings() {
        assert_eq!(normalize_string("---[Test].png"), "-test.png");
    }
}
