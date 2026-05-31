use std::collections::HashMap;
use clap::{Parser, Subcommand};

use crate::util::{
    command::{Commander, parse},
    fs,
    recorder,
};

mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the .rub file to run (optional if using config)
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Record user interactions into a .rub file
    Record {
        /// The output file for the recording
        #[arg(short, long, default_value = "recorded.rub")]
        output: String,
    },
    /// Run a .rub file
    Run {
        /// The .rub file to run
        #[arg(short, long)]
        file: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = util::config::boot();
    let client = util::client::new().await;

    match cli.command {
        Some(Commands::Record { output }) => {
            recorder::start_recording(client.clone(), output).await;
        }
        Some(Commands::Run { file }) => {
            let target_file = file.or(cli.file).unwrap_or(config.index_file.clone());
            run_script(client.clone(), &target_file).await;
        }
        None => {
            // Default behavior if no subcommand is provided
            let target_file = cli.file.unwrap_or(config.index_file.clone());
            run_script(client.clone(), &target_file).await;
        }
    }

    let _ = client.close().await;
}

async fn run_script(client: fantoccini::Client, file_path: &str) {
    let commands = match parse(fs::read_rub_file(file_path).as_str()) {
        Some(d) => d,
        None => {
            eprintln!("Failed to parse commands from file: {}", file_path);
            return;
        }
    };

    let mut commander = Commander::new(client, commands, HashMap::new());
    commander.exec().await;
}
