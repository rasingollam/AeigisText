use aegis_text::{analyze_text, AnalyseOptions};
use clap::Parser;
use serde_json;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about = "AegisText - lightweight text analysis for AI agents")]
struct Cli {
    /// Input file (if omitted, read from stdin)
    #[arg(long)]
    file: Option<String>,

    /// Include keyword extraction (top 5)
    #[arg(long)]
    keywords: bool,

    /// Include basic sentiment analysis
    #[arg(long)]
    sentiment: bool,

    /// Include basic summary (first 1-2 sentences)
    #[arg(long)]
    summary: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut input = String::new();
    if let Some(path) = cli.file.as_deref() {
        match fs::read_to_string(path) {
            Ok(s) => input = s,
            Err(e) => {
                eprintln!("Error reading file {}: {}", path, e);
                std::process::exit(2);
            }
        }
    } else {
        if let Err(e) = io::stdin().read_to_string(&mut input) {
            eprintln!("Error reading stdin: {}", e);
            std::process::exit(2);
        }
    }

    if input.trim().is_empty() {
        eprintln!("Empty input");
        std::process::exit(3);
    }

    let opts = AnalyseOptions {
        keywords: cli.keywords,
        sentiment: cli.sentiment,
        summary: cli.summary,
    };

    let result = analyze_text(&input, &opts);

    match serde_json::to_string_pretty(&result) {
        Ok(js) => {
            println!("{}", js);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to serialize JSON output: {}", e);
            std::process::exit(1);
        }
    }
}
