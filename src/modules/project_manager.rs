#![allow(unused_variables)]

use clap::{Parser, Subcommand};
use log;
use std::{fs, io};

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    dir: String,
    // owner: Option<String>,
    // repo: Option<String>,
    #[command(subcommand)]
    command: PrjCommands,
}

#[derive(Subcommand)]
enum PrjCommands {
    Ls { search: String },
    Rm { owner: String, repo: String },
    Add { owner: String, repo: String },
}

impl Cli {
    pub fn run(&self) -> Result<(), io::Error> {
        match &self.command {
            PrjCommands::Ls { search } => {
                for entry in fs::read_dir(&self.dir)? {
                    let path = entry?.path();
                    let path = path.to_str().unwrap();
                    if path.contains(search) {
                        log::info!("{}", path);
                    }
                }
            }
            PrjCommands::Rm { owner, repo } => {
                let request_url = format!(
                    "https://github.com/{owner}/{repo}",
                    owner = owner,
                    repo = repo,
                );
            }
            PrjCommands::Add { owner, repo } => {
                let request_url = format!(
                    "https://github.com/{owner}/{repo}",
                    owner = owner,
                    repo = repo,
                );
            }
        }
        Ok(())
    }
}

pub fn run() {
    let cli = Cli::parse();
    cli.run().unwrap();
}
