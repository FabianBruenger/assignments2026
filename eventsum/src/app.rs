use crate::event::Event;
use crate::result::SummaryResult;
use log::{debug, info, error,warn};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct App {
    // Vector of valid events
    events: Vec<Event>,
    // HashMap to track user counts
    user_counts: HashMap<String, usize>,
    // Placeholder: will hold Result struct later
    result: SummaryResult,
}

impl App {
    /// Creates a new App instance
    pub fn new() -> Self {
        App {
            events: Vec::new(),
            user_counts: HashMap::new(),
            result: SummaryResult::new(),
        }
    }
    
    /// Increments the count for a user
    fn increment_user_count(&mut self, user: &str) {
        *self.user_counts.entry(user.to_string()).or_insert(0) += 1;
    }

    // TODO: refactor handling logic to 1 private method to avoid code duplication

    /// Reads events from a file at the given path
    /// Returns an error if the file cannot be read (exit code 2)
    pub fn read_from_file(&mut self, path: &str) -> io::Result<()> {
        info!("Opening file: {}", path);
        let file = File::open(Path::new(path))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            // Ignore blank lines
            if line.trim().is_empty() {
                warn!("Skipping blank line");
                self.result.increment_total_lines();
                self.result.increment_bad_lines();
                continue;
            }
            debug!("Read line {}: {}", self.result.total_lines, line);

            // Now create the event from line. If the line is invalid (JSON invalid) log for now.
            // If event is valid (no empty fields) add to events vector
            match Event::from_json_line(&line) {
                Some(event) => {
                    if event.is_valid() {
                        self.events.push(event.clone());
                        self.increment_user_count(&event.user);
                        self.result.increment_events();
                        self.result.update_level_counts(event.level);

                    } else {
                        error!(
                            "Invalid event at line {}: missing required fields",
                            self.result.total_lines
                        );
                        self.result.increment_bad_lines();
                    }
                }
                None => {
                    error!("Failed to parse JSON at line {}", self.result.total_lines);
                    self.result.increment_bad_lines();
                    continue;
                }
            }
            self.result.increment_total_lines();
        }

        info!("Finished reading {} lines from file", self.result.total_lines);
        Ok(())
    }

    /// Reads events from stdin
    /// Returns an error if stdin cannot be read (exit code 2)
    pub fn read_from_stdin(&mut self) -> io::Result<()> {
        info!("Reading from stdin");
        let stdin = io::stdin();
        let reader = stdin.lock();

        let mut line_count = 0;
        for line in reader.lines() {
            let line = line?;
            // Ignore blank lines
            if line.trim().is_empty() {
                debug!("Skipping blank line");
            }
            line_count += 1;
            debug!("Read line {}: {}", line_count, line);
            // TODO: Process line (parse JSON, validate, etc.)
        }

        info!("Finished reading {} lines from stdin", line_count);
        Ok(())
    }
}
