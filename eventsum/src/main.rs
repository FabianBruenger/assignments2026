mod app;

use clap::Parser;
use std::process;

#[derive(Parser)]
#[command(name = "eventsum")]
#[command(about = "Parses event log (JSON Lines) and produces a summary report", long_about = None)]
struct Cli {
    /// Input file path. If omitted, reads from stdin
    #[arg(short, long)]
    input: Option<String>,

    /// Pretty-print the output JSON
    #[arg(long)]
    pretty: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut app = app::App::new();

    let result = match cli.input {
        Some(path) => {
            // Read from file
            app.read_from_file(&path)
        }
        None => {
            // Read from stdin
            app.read_from_stdin()
        }
    };

    // Handle errors (exit code 2 for input read failures)
    if let Err(e) = result {
        eprintln!("Error reading input: {}", e);
        process::exit(2);
    }

    // TODO: Process events and generate output
    // For now, exit with success
    process::exit(0);
}
