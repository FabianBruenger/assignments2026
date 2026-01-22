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
    
    /// Computes summary from a list of events
    pub fn from_events(events: &[Event], total_lines: usize, bad_lines: usize) -> Self {
        let mut result = SummaryResult::new();
        result.total_lines = total_lines;
        result.bad_lines = bad_lines;
        result.events = events.len();
        
        // Count by level
        for event in events {
            match event.level {
                Level::Info => result.by_level.info += 1,
                Level::Warn => result.by_level.warn += 1,
                Level::Error => result.by_level.error += 1,
            }
        }
        
        // Count by user
        let mut user_counts: HashMap<String, usize> = HashMap::new();
        for event in events {
            *user_counts.entry(event.user.clone()).or_insert(0) += 1;
        }
        
        // Get top 3 users (descending by count, then ascending by username)
        let mut user_vec: Vec<UserCount> = user_counts
            .into_iter()
            .map(|(user, count)| UserCount { user, count })
            .collect();
        user_vec.sort_by(|a, b| {
            b.count.cmp(&a.count).then_with(|| a.user.cmp(&b.user))
        });
        result.top_users = user_vec.into_iter().take(3).collect();
        
        // Calculate p95 duration
        if !events.is_empty() {
            let mut durations: Vec<u64> = events.iter().map(|e| e.duration_ms).collect();
            durations.sort_unstable();
            let n = durations.len();
            let rank = ((0.95 * n as f64).ceil() as usize).max(1);
            result.p95_duration_ms = durations[rank - 1];
        }
        
        // Find outlier (event with max duration)
        result.outlier = events.iter().max_by_key(|e| e.duration_ms).cloned();
        
        result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_result() {
        let result = SummaryResult::from_events(&[], 0, 0);
        assert_eq!(result.events, 0);
        assert_eq!(result.p95_duration_ms, 0);
        assert!(result.outlier.is_none());
    }
    
    #[test]
    fn test_single_event() {
        let event = Event {
            ts: "2026-01-19T12:00:01Z".to_string(),
            level: Level::Info,
            user: "alice".to_string(),
            action: "test".to_string(),
            duration_ms: 100,
        };
        let result = SummaryResult::from_events(&[event], 1, 0);
        assert_eq!(result.events, 1);
        assert_eq!(result.p95_duration_ms, 100);
        assert_eq!(result.by_level.info, 1);
    }
}
