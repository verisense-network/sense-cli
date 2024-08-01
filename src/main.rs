use clap::{Parser, Subcommand};
use std::collections::HashSet;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate ASCII art for a word
    Generate {
        /// The word to generate ASCII art for
        word: String,
    },
    /// List available letters
    List,
}

fn get_letter_ascii(c: char) -> Vec<String> {
    match c {
        'S' => vec![
            " ____  ".to_string(),
            "/ ___| ".to_string(),
            "\\___ \\ ".to_string(),
            " ___) |".to_string(),
            "|____/ ".to_string(),
        ],
        'E' => vec![
            " _____ ".to_string(),
            "| ____|".to_string(),
            "|  _|  ".to_string(),
            "| |___ ".to_string(),
            "|_____|".to_string(),
        ],
        'N' => vec![
            " _   _ ".to_string(),
            "| \\ | |".to_string(),
            "|  \\| |".to_string(),
            "| |\\  |".to_string(),
            "|_| \\_|".to_string(),
        ],
        // Add more letters as needed
        _ => vec![" ", " ", " ", " ", " "]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
    }
}

fn generate_word_ascii(word: &str) -> String {
    let letters: Vec<Vec<String>> = word.to_uppercase().chars().map(get_letter_ascii).collect();
    let height = letters[0].len();

    (0..height)
        .map(|i| {
            letters
                .iter()
                .map(|letter| letter[i].clone())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn list_available_letters() -> Vec<char> {
    let mut available = HashSet::new();
    for c in 'A'..='Z' {
        if get_letter_ascii(c) != get_letter_ascii(' ') {
            available.insert(c);
        }
    }
    let mut result: Vec<char> = available.into_iter().collect();
    result.sort();
    result
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Generate { word }) => {
            println!("Generating ASCII art for: {}", word);
            println!("{}", generate_word_ascii(word));
        }
        Some(Commands::List) => {
            println!("Available letters:");
            for c in list_available_letters() {
                println!("{}", c);
            }
        }
        None => {
            println!("No command specified. Use --help for usage information.");
        }
    }
}
