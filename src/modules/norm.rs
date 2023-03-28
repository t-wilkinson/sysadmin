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

/// Normalize the given path and optionally recurse if the path is a directory
///
/// We want to move the path because we are renaming the file.
pub fn normalize_path(path: String, normalize_contents: bool) -> String {
    let normalized_path = get_normalized_path(&path);
    let _ = fs::rename(&path, &normalized_path);
    if normalize_contents {
        let _ = normalize_dir(Path::new(&normalized_path), normalize_contents);
    }
    // match fs::rename(&path, &normalized_path) {
    //     Ok(_) => {
    //         if normalize_contents {
    //             let _ = normalize_dir(Path::new(&normalized_path), normalize_contents);
    //         }
    //     }
    //     Err(err) => eprintln!(
    //         "Error: could not rename `{}` to `{}` {}",
    //         &path, &normalized_path, err
    //     ),
    // }

    normalized_path
}

fn normalize_dir(path: &Path, normalize_contents: bool) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let path_buf = entry?.path();
        let path = String::from(path_buf.to_str().unwrap());
        normalize_path(path, path_buf.is_dir() && normalize_contents);
    }

    Ok(())
}

fn normalize_files(cli: &Cli, paths: &Vec<String>) -> Vec<String> {
    let mut normalized_paths = Vec::new();

    for path in paths {
        match fs::metadata(path) {
            Ok(md) => {
                let normalized_path =
                    normalize_path(String::from(path), md.is_dir() && cli.recursive);
                normalized_paths.push(normalized_path);
            }
            Err(_) => {
                eprintln!("Error: could not find metadata for `{}`", path);
            }
        }
    }

    normalized_paths
}

fn normalize_stdin() {
    for line in io::stdin().lines() {
        println!("{}", get_normalized_path(&line.unwrap()));
    }
}

#[derive(Parser)]
pub struct Cli {
    paths: Option<Vec<String>>,
    #[arg(short, long, default_value_t = true)]
    recursive: bool,
}

impl Cli {
    pub fn run(&self) {
        if let Some(paths) = &self.paths {
            normalize_files(self, paths);
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
