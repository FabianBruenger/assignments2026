use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Event {
    #[serde(rename = "type")]
    event_type: Option<String>,
    timestamp: Option<String>,
    level: Option<String>,
    message: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

struct EventSummary {
    total_events: usize,
    events_by_type: HashMap<String, usize>,
    events_by_level: HashMap<String, usize>,
}

impl EventSummary {
    fn new() -> Self {
        EventSummary {
            total_events: 0,
            events_by_type: HashMap::new(),
            events_by_level: HashMap::new(),
        }
    }

    fn add_event(&mut self, event: &Event) {
        self.total_events += 1;

        if let Some(event_type) = &event.event_type {
            *self.events_by_type.entry(event_type.clone()).or_insert(0) += 1;
        }

        if let Some(level) = &event.level {
            *self.events_by_level.entry(level.clone()).or_insert(0) += 1;
        }
    }

    fn print_report(&self) {
        println!("=== Event Log Summary Report ===");
        println!();
        println!("Total Events: {}", self.total_events);
        println!();

        if !self.events_by_type.is_empty() {
            println!("Events by Type:");
            let mut types: Vec<_> = self.events_by_type.iter().collect();
            types.sort_by(|a, b| b.1.cmp(a.1));
            for (event_type, count) in types {
                println!("  {}: {}", event_type, count);
            }
            println!();
        }

        if !self.events_by_level.is_empty() {
            println!("Events by Level:");
            let mut levels: Vec<_> = self.events_by_level.iter().collect();
            levels.sort_by(|a, b| b.1.cmp(a.1));
            for (level, count) in levels {
                println!("  {}: {}", level, count);
            }
            println!();
        }

        println!("================================");
    }
}

fn parse_event_log(file_path: &Path) -> Result<EventSummary, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut summary = EventSummary::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<Event>(&line) {
            Ok(event) => {
                summary.add_event(&event);
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse line {}: {}", line_num + 1, e);
            }
        }
    }

    Ok(summary)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <event_log_file>", args[0]);
        eprintln!();
        eprintln!("Parse a JSON Lines event log and produce a summary report.");
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    if !file_path.exists() {
        eprintln!("Error: File '{}' not found", file_path.display());
        std::process::exit(1);
    }

    match parse_event_log(file_path) {
        Ok(summary) => {
            summary.print_report();
        }
        Err(e) => {
            eprintln!("Error processing file: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_parsing() {
        let json = r#"{"type":"test_event","level":"info","message":"Test message"}"#;
        let event: Event = serde_json::from_str(json).unwrap();

        assert_eq!(event.event_type, Some("test_event".to_string()));
        assert_eq!(event.level, Some("info".to_string()));
        assert_eq!(event.message, Some("Test message".to_string()));
    }

    #[test]
    fn test_event_summary() {
        let mut summary = EventSummary::new();

        let event1 = Event {
            event_type: Some("login".to_string()),
            timestamp: Some("2026-01-22T07:00:00Z".to_string()),
            level: Some("info".to_string()),
            message: Some("User logged in".to_string()),
            extra: HashMap::new(),
        };

        let event2 = Event {
            event_type: Some("login".to_string()),
            timestamp: Some("2026-01-22T07:05:00Z".to_string()),
            level: Some("info".to_string()),
            message: Some("User logged in".to_string()),
            extra: HashMap::new(),
        };

        let event3 = Event {
            event_type: Some("error".to_string()),
            timestamp: Some("2026-01-22T07:10:00Z".to_string()),
            level: Some("error".to_string()),
            message: Some("Error occurred".to_string()),
            extra: HashMap::new(),
        };

        summary.add_event(&event1);
        summary.add_event(&event2);
        summary.add_event(&event3);

        assert_eq!(summary.total_events, 3);
        assert_eq!(summary.events_by_type.get("login"), Some(&2));
        assert_eq!(summary.events_by_type.get("error"), Some(&1));
        assert_eq!(summary.events_by_level.get("info"), Some(&2));
        assert_eq!(summary.events_by_level.get("error"), Some(&1));
    }

    #[test]
    fn test_parse_event_log() {
        let summary = parse_event_log(Path::new("sample_events.jsonl")).unwrap();
        assert_eq!(summary.total_events, 10);
    }
}
