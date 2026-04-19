/// Morse Code Converter – CLI entry point.
///
/// Usage:
///   morse encode [--play] <TEXT…>
///   morse decode [--play] <MORSE…>
///
/// Examples:
///   morse encode "Hello World"
///   morse encode --play SOS
///   morse decode "... --- ..."
///   morse decode --play ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."

mod audio;
mod morse;

use clap::{Parser, Subcommand};

// ── CLI definition ────────────────────────────────────────────────────────────

/// Morse Code Converter  –  encode / decode Morse code with optional audio
#[derive(Parser)]
#[command(
    name    = "morse",
    version = "2.0.0",
    author  = "Tauasa Timoteo",
    about   = "Convert text ↔ Morse code and optionally play tones",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Encode plain text to Morse code
    Encode {
        /// Play audio tones while printing output
        #[arg(short, long)]
        play: bool,

        /// Text to encode (multiple words are joined with a space)
        #[arg(required = true, num_args = 1..)]
        text: Vec<String>,
    },

    /// Decode Morse code to plain text
    Decode {
        /// Play audio tones while printing output
        #[arg(short, long)]
        play: bool,

        /// Morse code to decode.
        /// Use quotes to pass dots, dashes, and slashes as a single argument.
        /// Letter separator: space.  Word separator: ' / '  (space-slash-space).
        #[arg(required = true, num_args = 1..)]
        morse: Vec<String>,
    },
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Encode { play, text } => {
            let input = text.join(" ");
            run_encode(&input, play);
        }
        Command::Decode { play, morse } => {
            let input = morse.join(" ");
            run_decode(&input, play);
        }
    }
}

// ── Subcommand handlers ───────────────────────────────────────────────────────

fn run_encode(text: &str, play: bool) {
    print_box_top();
    print_row("Input  (Text)", text);
    print_divider();

    match morse::encode(text) {
        Ok(encoded) => {
            print_row("Output (Morse)", &encoded);
            print_box_bottom();

            if play {
                play_tones(&encoded);
            }
        }
        Err(e) => {
            print_box_bottom();
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run_decode(morse_input: &str, play: bool) {
    print_box_top();
    print_row("Input  (Morse)", morse_input);
    print_divider();

    match morse::decode(morse_input) {
        Ok(decoded) => {
            print_row("Output (Text)", &decoded);
            print_box_bottom();

            if play {
                play_tones(morse_input);
            }
        }
        Err(e) => {
            print_box_bottom();
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn play_tones(morse: &str) {
    eprintln!("♪  Playing…");
    if let Err(e) = audio::play(morse) {
        eprintln!("audio error: {e}");
    } else {
        eprintln!("♪  Done.");
    }
}

// ── Pretty-print helpers ──────────────────────────────────────────────────────

const WIDTH: usize = 60;

fn print_box_top() {
    println!("┌{}┐", "─".repeat(WIDTH));
}

fn print_box_bottom() {
    println!("└{}┘", "─".repeat(WIDTH));
}

fn print_divider() {
    println!("├{}┤", "─".repeat(WIDTH));
}

/// Prints a labelled row, wrapping long values across multiple lines.
fn print_row(label: &str, value: &str) {
    // Inner width minus "│ " prefix and " │" suffix = WIDTH - 2
    let inner = WIDTH - 2;
    let label_line = format!("{label}:");
    println!("│ {:<inner$} │", label_line);

    // Word-wrap the value
    for line in wrap(value, inner) {
        println!("│ {:<inner$} │", line);
    }
}

/// Naive word-wrap: split on whitespace, build lines up to `max_width` chars.
fn wrap(text: &str, max_width: usize) -> Vec<String> {
    if text.len() <= max_width {
        return vec![text.to_string()];
    }

    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= max_width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
