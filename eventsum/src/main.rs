mod app;

use clap::Parser;
use log::{debug, error};
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
    // Initialize logger (set RUST_LOG=debug for detailed output)
    env_logger::init();
    
    let cli = Cli::parse();
    let mut app = app::App::new();
    
    debug!("Starting eventsum with pretty={}", cli.pretty);

    let result = match cli.input {
        Some(ref path) => {
            debug!("Reading from file: {}", path);
            app.read_from_file(path)
        }
        None => {
            debug!("Reading from stdin");
            app.read_from_stdin()
        }
    };

    // Handle errors (exit code 2 for input read failures)
    if let Err(e) = result {
        error!("Error reading input: {}", e);
        eprintln!("Error reading input: {}", e);
        process::exit(2);
    }
    
    debug!("Successfully read input");

    // TODO: Process events and generate output
    // For now, exit with success
    process::exit(0);
}
