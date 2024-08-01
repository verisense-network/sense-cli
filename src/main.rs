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
        'A' => vec!["  ___  ", " / _ \\ ", "/ /_\\ \\", "|  _  |", "|_| |_|"],
        'B' => vec![" ____  ", "|  _ \\ ", "| |_) |", "|  _ < ", "|_|_\\_\\"],
        'C' => vec!["  ____ ", " / ___|", "| |    ", "| |    ", " \\____|"],
        'D' => vec![" ____  ", "|  _ \\ ", "| | | |", "| |_| |", "|____/ "],
        'E' => vec![" _____ ", "| ____|", "|  _|  ", "| |___ ", "|_____|"],
        'F' => vec![" _____ ", "|  ___|", "| |_   ", "|  _|  ", "|_|    "],
        'G' => vec!["  ____ ", " / ___|", "| |  _ ", "| |_| |", " \\____|"],
        'H' => vec![" _   _ ", "| | | |", "| |_| |", "|  _  |", "|_| |_|"],
        'I' => vec![" _____ ", "|_   _|", "  | |  ", "  | |  ", " _|_|_ "],
        'J' => vec!["     _ ", "    | |", " _  | |", "| |_| |", " \\___/ "],
        'K' => vec![" _   __", "| | / /", "| |/ / ", "|    \\ ", "|_|\\_\\"],
        'L' => vec![" _     ", "| |    ", "| |    ", "| |___ ", "|_____|"],
        'M' => vec![" __  __", "|  \\/  |", "| |\\/| |", "| |  | |", "|_|  |_|"],
        'N' => vec![" _   _ ", "| \\ | |", "|  \\| |", "| |\\  |", "|_| \\_|"],
        'O' => vec!["  ___  ", " / _ \\ ", "| | | |", "| |_| |", " \\___/ "],
        'P' => vec![" ____  ", "|  _ \\ ", "| |_) |", "|  __/ ", "|_|    "],
        'Q' => vec!["  ___  ", " / _ \\ ", "| | | |", "| |_| |", " \\__\\_\\"],
        'R' => vec![" ____  ", "|  _ \\ ", "| |_) |", "|  _ < ", "|_| \\_\\"],
        'S' => vec![" ____  ", "/ ___| ", "\\___ \\ ", " ___) |", "|____/ "],
        'T' => vec![" _____ ", "|_   _|", "  | |  ", "  | |  ", "  |_|  "],
        'U' => vec![" _   _ ", "| | | |", "| | | |", "| |_| |", " \\___/ "],
        'V' => vec![
            "__     __",
            "\\ \\   / /",
            " \\ \\ / / ",
            "  \\ V /  ",
            "   \\_/   ",
        ],
        'W' => vec![
            "__        __",
            "\\ \\      / /",
            " \\ \\ /\\ / / ",
            "  \\ V  V /  ",
            "   \\_/\\_/   ",
        ],
        'X' => vec!["__  __", "\\ \\/ /", " \\  / ", " /  \\ ", "/_/\\_\\"],
        'Y' => vec!["__   __", "\\ \\ / /", " \\ V / ", "  | |  ", "  |_|  "],
        'Z' => vec![" _____", "|__  /", "  / / ", " / /_ ", "/____|"],
        _ => vec![" ", " ", " ", " ", " "],
    }
    .iter()
    .map(|s| s.to_string())
    .collect()
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
