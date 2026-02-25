use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::json;
use std::env;
use std::thread;
use std::time::Duration;
use football_match_parser::Match;

/// Command-line interface for interacting with the Football Match Parser.
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

/// generates title with Gemini API
fn generate_ai_headline(m: &Match) -> Result<String> {
    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => return Ok("AI generation skipped (no API key found)".to_string()),
    };

    let prompt = format!(
        "Act as a professional sports editor. Write one realistic, and catchy but creative news headline for this football match. \
         It should sound like a real headline from BBC Sport or ESPN. Keep it under 10 words. \
         Teams: {} vs {}. Score: {:?} - {:?}. Status: {:?}. Stadium: {:?}",
        m.home_team, m.away_team, m.home_score, m.away_score, m.status, m.stadium
    );

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", api_key);

    let payload = json!({
        "contents": [{
            "parts": [{"text": prompt}]
        }]
    });

    let client = reqwest::blocking::Client::new();
    let res: serde_json::Value = client.post(&url)
        .json(&payload)
        .send()?
        .json()?;
    if let Some(err) = res.get("error") {
        return Err(anyhow::anyhow!("API Error: {}", err["message"]));
    }
    let headline = res["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .context("Failed to parse AI response (можливо, ліміт запитів)")?
        .trim()
        .to_string();

    Ok(headline)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { path } => {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read input file '{}'", path))?;

            for (i, line) in content.lines().enumerate() {
                match football_match_parser::parse_match_line(line) {
                    Ok(m) => {
                        println!("Line {} OK: {:?}", i + 1, m);

                        println!("Generating title...");
                        match generate_ai_headline(&m) {
                            Ok(headline) => {
                                println!("TITLE: {}", headline);
                                thread::sleep(Duration::from_secs(4));
                            },
                            Err(e) => {
                                println!("АІ Error: {}", e);
                                thread::sleep(Duration::from_secs(4));
                            }
                        }
                    },
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