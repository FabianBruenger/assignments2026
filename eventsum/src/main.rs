use clap::Parser;
use log::{debug, error,info};
use std::process;

// Internal modules
mod app;
mod event;
mod cli;
mod result;

// TODO; when reading from std in, need reset at some point. Otherwise overflow

fn main() {
    // Initialize logger (set RUST_LOG=debug for detailed output)
    env_logger::init();
    
    let cli = crate::cli::Cli::parse();
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
        process::exit(2);
    }

    // Finalize: compute top users, p95, and outlier
    app.finalize();
    
    // Generate and output JSON
    match app.get_result().to_json(cli.pretty) {
        Ok(json) => {
            info!("Pretty Result:{}", json);
        }
        Err(e) => {
            error!("Failed to serialize result to JSON: {}", e);
            process::exit(1);
        }
    }
    
    // Exit with success
    process::exit(0);
}


