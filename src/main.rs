use clap::{Parser, Subcommand};

mod commands;
mod object;
mod utils;

#[derive(Parser)]
#[command(name = "gut")]
#[command(version = "0.1")]
#[command(about = "Mini VCS em Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add {
        files: Vec<String>,
    },
    Commit {
        #[arg(short, long)]
        message: String,
    },
    Log,
    Status,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => {
            commands::init::run();
            Ok(())
        }

        Commands::Add { files } => {
            commands::add::run(files)
        }

        Commands::Commit { message } => {
            commands::commit::run(message)
        }

        Commands::Log => {
            commands::log::run()
        }

        Commands::Status => {
            commands::status::run()
        }
    };

    if let Err(err) = result {
        eprintln!("error: {}", err);
    }
}