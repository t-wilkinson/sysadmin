mod lib;
use super::lib::{run, Cli};

fn main() {
    let cli = Cli::parse();
    run(cli);
}
