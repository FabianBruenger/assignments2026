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
    
    /// Finalizes the result by computing top users, p95, and outlier
    pub fn finalize(&mut self) {
        info!("Finalizing results: computing top users, p95, and outlier");
        self.result.compute_top_users(&self.user_counts);
        self.result.compute_p95_duration(&self.events);
        self.result.compute_outlier(&self.events);
    }
    
    /// Returns a reference to the result
    pub fn get_result(&self) -> &SummaryResult {
        &self.result
    }
    
    /// Processes a single line (helper method)
    fn process_line(&mut self, line: &str) {
        // Ignore blank lines
        if line.trim().is_empty() {
            warn!("Skipping blank line but counting it");
            self.result.increment_total_lines();
            self.result.increment_bad_lines();
            return;
        }
        
        debug!("Processing line {}: {}", self.result.total_lines + 1, line);
        
        // Parse event from JSON
        match Event::from_json_line(line) {
            Some(event) => {
                if event.is_valid() {
                    self.events.push(event.clone());
                    self.increment_user_count(&event.user);
                    self.result.increment_events();
                    self.result.update_level_counts(event.level);
                    self.result.increment_total_lines();
                } else {
                    error!(
                        "Invalid event at line {}: missing required fields",
                        self.result.total_lines + 1
                    );
                    self.result.increment_bad_lines();
                    self.result.increment_total_lines();
                }
            }
            None => {
                error!("Failed to parse JSON at line {}", self.result.total_lines + 1);
                self.result.increment_bad_lines();
                self.result.increment_total_lines();
            }
        }
    }

    /// Reads events from a file at the given path
    /// Returns an error if the file cannot be read (exit code 2)
    pub fn read_from_file(&mut self, path: &str) -> io::Result<()> {
        info!("Opening file: {}", path);
        let file = File::open(Path::new(path))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            self.process_line(&line);
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

        for line in reader.lines() {
            let line = line?;
            self.process_line(&line);
        }

        info!("Finished reading {} lines from stdin", self.result.total_lines);
        Ok(())
    }
}

//  TODO: Test Overflow