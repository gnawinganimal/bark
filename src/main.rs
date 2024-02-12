use std::fs;

use calender::Calender;
use cli::{Cli, Cmd};
use clap::Parser;

pub mod calender;
pub mod cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Cmd::Watch { path } => {
                let src = fs::read_to_string(path)
                    .expect("Could not read file");
                let cal = toml::from_str::<Calender>(&src)
                    .expect("Could not parse toml");

                println!("Watching {}...\n", cal.name);
                
                for event in cal.event {
                    println!("Launching event {}", event.name);

                    
                }
            }
        };
    } else {
        todo!("Implement default behaviour");
    }    
}
