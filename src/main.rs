use cli::{Cli, Cmd};
use clap::Parser;

pub mod cli;

fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Cmd::Watch { path: _ } => {
                todo!("Implement watch functionality")
            }
        }
    } else {
        todo!("Implement default behaviour");
    }    
}
