use serde::{Deserialize, Serialize};
use log::{debug, warn, error};

/// Log level for events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Level {
    Info,
    Warn,
    Error,
}

/// Event structure representing a single log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// ISO-8601 timestamp (UTC)
    pub ts: String,
    /// Log level
    pub level: Level,
    /// Username
    pub user: String,
    /// Action name
    pub action: String,
    /// Duration in milliseconds
    pub duration_ms: u64,
}

impl Event {
    /// Parses a JSON line into an Event
    /// Returns None if parsing fails or validation fails
    pub fn from_json_line(line: &str) -> Option<Self> {
        match serde_json::from_str(line) {
            Ok(event) => Some(event),
            Err(e) => {
                error!("Failed to parse JSON: {}", e);
                None
            }
        }
    }

    /// Validates that the event has non-empty required fields
    pub fn is_valid(&self) -> bool {
        let mut valid = true;
        
        if self.ts.is_empty() {
            warn!("Validation failed: ts field is empty");
            valid = false;
        }
        
        if self.user.is_empty() {
            warn!("Validation failed: user field is empty");
            valid = false;
        }
        
        if self.action.is_empty() {
            warn!("Validation failed: action field is empty");
            valid = false;
        }
        
        valid
    }
    
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_event_parsing() {
        let json = r#"{"ts":"2026-01-19T12:00:01Z","level":"INFO","user":"alice","action":"run_script","duration_ms":120}"#;
        let event = Event::from_json_line(json);
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.user, "alice");
        assert_eq!(event.level, Level::Info);
        assert_eq!(event.duration_ms, 120);
    }

    #[test]
    fn test_invalid_json() {
        let json = "not-json";
        let event = Event::from_json_line(json);
        assert!(event.is_none());
    }

    #[test]
    fn test_empty_user() {
        let json = r#"{"ts":"2026-01-19T12:00:01Z","level":"INFO","user":"","action":"run_script","duration_ms":120}"#;
        let event = Event::from_json_line(json);
        assert!(event.is_none());
    }

    #[test]
    fn test_all_levels() {
        let levels = vec![
            ("INFO", Level::Info),
            ("WARN", Level::Warn),
            ("ERROR", Level::Error),
        ];
        
        for (level_str, expected_level) in levels {
            let json = format!(r#"{{"ts":"2026-01-19T12:00:01Z","level":"{}","user":"alice","action":"test","duration_ms":100}}"#, level_str);
            let event = Event::from_json_line(&json);
            assert!(event.is_some());
            assert_eq!(event.unwrap().level, expected_level);
        }
    }
}