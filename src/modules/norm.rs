/// Normalizes file paths
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::{fs, io};

lazy_static! {
    static ref RE_SIMPLE_CHARS: Regex = Regex::new(r"[^./\w\s-]").unwrap();
    static ref RE_DUPLICATE_CHARS: Regex = Regex::new(r"[-\s_]+").unwrap();
}

pub fn get_normalized_path(string: &str) -> String {
    let string = string.to_lowercase();
    let string = string.trim();
    let string = RE_SIMPLE_CHARS.replace_all(string, "").to_string();
    let string = RE_DUPLICATE_CHARS.replace_all(&string, "-").to_string();
    string
}

// TODO: given that the path changes, we should move the path (not pass a reference)
pub fn normalize_path(path: &str, normalize_contents: bool) -> String {
    let normalized_path = get_normalized_path(path);
    match fs::rename(path, &normalized_path) {
        Ok(_) => {
            if normalize_contents {
                let _ = normalize_dir(Path::new(&normalized_path), normalize_contents);
            }
        }
        Err(_) => eprintln!(
            "Error: could not rename `{}` to `{}`",
            path, &normalized_path
        ),
    }

    normalized_path
}

fn normalize_dir(path: &Path, normalize_contents: bool) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let path_buf = entry?.path();
        let path = path_buf.to_str().unwrap();
        normalize_path(path, path_buf.is_dir() && normalize_contents);
    }

    Ok(())
}

fn normalize_files(cli: &Cli, paths: &Vec<String>) -> Result<(), io::Error> {
    for path in paths {
        match fs::metadata(path) {
            Ok(md) => {
                normalize_path(path, md.is_dir() && cli.recursive);
            }
            Err(_) => {
                eprintln!("Error: could not find metadata for `{}`", path);
            }
        }
    }

    Ok(())
}

fn normalize_stdin() {
    for line in io::stdin().lines() {
        println!("{}", get_normalized_path(&line.unwrap()));
    }
}

#[derive(Parser)]
pub struct Cli {
    paths: Option<Vec<String>>,
    #[arg(short, long)]
    recursive: bool,
}

impl Cli {
    pub fn run(&self) {
        if let Some(paths) = &self.paths {
            normalize_files(self, paths).unwrap();
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
    fn get_normalized_paths() {
        assert_eq!(get_normalized_path("---[Test].png"), "-test.png");
    }
}
