use clap::Parser;

#[derive(Parser)]
#[command(name = "eventsum")]
#[command(about = "Parses event log (JSON Lines) and produces a summary report")]
#[command(long_about = "Parses event log (JSON Lines) and produces a summary report.\n\nLogging:\n  Set RUST_LOG environment variable to control log output:\n  - RUST_LOG=error  : Errors only\n  - RUST_LOG=info   : Major operations\n  - RUST_LOG=debug  : Detailed line processing\n  - RUST_LOG=trace  : Maximum verbosity\n\nExample:\n  RUST_LOG=info eventsum --input events.jsonl")]
pub struct Cli {
    /// Input file path. If omitted, reads from stdin
    #[arg(short, long)]
    pub input: Option<String>,

    /// Pretty-print the output JSON
    #[arg(long)]
    pub pretty: bool,
}