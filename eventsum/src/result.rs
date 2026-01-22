use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::event::{Event, Level};

/// User count for top users ranking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCount {
    pub user: String,
    pub count: usize,
}

/// Level counts structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCounts {
    #[serde(rename = "INFO")]
    pub info: usize,
    #[serde(rename = "WARN")]
    pub warn: usize,
    #[serde(rename = "ERROR")]
    pub error: usize,
}

/// Summary result structure for event processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryResult {
    /// Total non-blank lines read
    pub total_lines: usize,
    /// Count of invalid lines
    pub bad_lines: usize,
    /// Count of valid events
    pub events: usize,
    /// Counts per log level
    pub by_level: LevelCounts,
    /// Top users by event count
    pub top_users: Vec<UserCount>,
    /// 95th percentile of duration_ms
    pub p95_duration_ms: u64,
    /// Event with the largest duration_ms
    pub outlier: Option<Event>,
}

impl SummaryResult {
    /// Creates a new empty SummaryResult
    pub fn new() -> Self {
        SummaryResult {
            total_lines: 0,
            bad_lines: 0,
            events: 0,
            by_level: LevelCounts {
                info: 0,
                warn: 0,
                error: 0,
            },
            top_users: Vec::new(),
            p95_duration_ms: 0,
            outlier: None,
        }
    }
    
    /// Increments the total_lines counter
    pub fn increment_total_lines(&mut self) {
        self.total_lines += 1;
    }
    
    /// Increments the bad_lines counter    
    pub fn increment_bad_lines(&mut self) {
        self.bad_lines += 1;
    }

    /// Increments the events counter
    pub fn increment_events(&mut self) {
        self.events += 1;
    }

    /// Updates level counts based on the event's level
    pub fn update_level_counts(&mut self, level: Level) {
        match level {
            Level::Info => self.by_level.info += 1,
            Level::Warn => self.by_level.warn += 1,
            Level::Error => self.by_level.error += 1,
        }
    }
    
    /// Serializes to JSON string
    pub fn to_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
    }
}


