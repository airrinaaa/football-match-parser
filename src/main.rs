use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

///Command-line interface for interacting with the Football Match Parser.
#[derive(Parser)]
#[command(
    name = "football_match_parser",
    about = "Parse football match data",
    long_about = "A CLI tool that parses football match information from text files.\n\
                  Each line contains date, time, teams, score, and optional stadium and status.\n\
                  Use `parse <file>` to extract structured data or `credits` to view author info."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse { path: String },
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Parse { path } => {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read input file '{}'", path))?;

            for (i, line) in content.lines().enumerate() {
                match football_match_parser::parse_match_line(line) {
                    Ok(m) => println!("Line {} OK: {:?}", i + 1, m),
                    Err(e) => println!("Line {} ERROR: {}", i + 1, e),
                }
            }
        }
        Commands::Credits => {
            println!("Football Match Parser by Iryna Rychok");
        }
    }

    Ok(())
}
