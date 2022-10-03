use async_process::Command;
use clap::Parser;
use colored::Colorize;
use futures::{executor::block_on, future::join_all};
use log;
use std::io::{self, Write};

#[derive(Parser)]
pub struct Cli {
    server: String,
    commands: String,
    server_name: String,
}

async fn run_ssh_command(description: &str, server: &str, command: &str) {
    let ssh_command: String = format!("ssh {} {}", server, command);

    let output = Command::new("bash")
        .args(["-c", &ssh_command])
        .output()
        .await
        .expect("running command failed");

    println!("{}", format!("{}", description).bold());
    log::info!("Running ssh command \"{}\"", command);
    io::stdout().write_all(&output.stdout).unwrap();
    println!();
}

impl Cli {
    pub async fn run(&self) {
        let mut handles = Vec::<_>::new();
        for command in self.commands.split(',') {
            let server_name = &self.server_name;
            let handle = match command {
                "docker" => run_ssh_command("Docker", server_name, "docker ps"),
                "users" => run_ssh_command("Active Users", server_name, "w"),
                "storage" => run_ssh_command("Storage", server_name, "df -h | grep /vda"),
                _ => panic!("Do not understand command {}.", command),
            };

            handles.push(handle);
        }

        join_all(handles).await;
    }
}

pub fn run() {
    let cli = Cli::parse();
    block_on(cli.run());
}
