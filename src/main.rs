#![allow(unused)]
use clap::{Parser, Subcommand};
// use simple_logger::SimpleLogger;
mod lib;
mod modules;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Normalize stuff
    Norm(modules::norm::Cli),

    /// File sharing
    /// Web server that allows
    ///     - sending files to server and reading files from file
    ///     - password protection
    Fs(modules::file_sharing::Cli),

    /// Monitor server
    Monitor(modules::monitor::Cli),

    /// Projects
    Prj(modules::project_manager::Cli),
    /*
    /// Requests
    Req(modules::req::Cli),
    */
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut cfg = lib::config::load_config();
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
