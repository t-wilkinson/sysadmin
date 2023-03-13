#![allow(unused)]
use clap::{Parser, Subcommand};
// use simple_logger::SimpleLogger;
use sysadmin;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Normalize stuff
    Norm(sysadmin::norm::Cli),

    /// File sharing
    /// Web server that allows
    ///     - sending files to server and reading files from file
    ///     - password protection
    Fs(sysadmin::file_sharing::Cli),

    /// Monitor server
    Monitor(sysadmin::monitor::Cli),

    /// Projects
    Prj(sysadmin::project_manager::Cli),
    /*
    /// Requests
    Req(sysadmin::req::Cli),
    */
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut cfg = sysadmin::config::load_config();
    // SimpleLogger::new().init().unwrap();

    let cli = Cli::parse();
    match cli.command {
        Commands::Norm(cli) => {
            cli.run();
        }
        Commands::Fs(cli) => {
            cli.run();
        }
        Commands::Monitor(cli) => {
            cli.run();
        }
        Commands::Prj(cli) => {
            cli.run();
        } // Commands::Req(cli) => {
          //     cli.run();
          // }
    }

    Ok(())
}
