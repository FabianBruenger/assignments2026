use crate::event::{Event, Level};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    
    /// Computes top users from a HashMap of user counts
    /// Returns top 3 users sorted by count (descending), then by username (ascending)
    pub fn compute_top_users(&mut self, user_counts: &HashMap<String, usize>) {
        let mut top_3: Vec<UserCount> = Vec::with_capacity(3);
        
        for (user, &count) in user_counts.iter() {
            let new_user = UserCount {
                user: user.clone(),
                count,
            };
            
            if top_3.len() < 3 {
                // Vec not full yet, insert in sorted position
                let insert_pos = top_3.iter().position(|u| {
                    count > u.count || (count == u.count && user < &u.user)
                }).unwrap_or(top_3.len());
                top_3.insert(insert_pos, new_user);
            } else {
                // Vec is full, check if new user should replace the worst (last) one
                let last = &top_3[2];
                if count > last.count || (count == last.count && user < &last.user) {
                    // Find correct position and replace
                    top_3.pop(); // Remove last (worst)
                    let insert_pos = top_3.iter().position(|u| {
                        count > u.count || (count == u.count && user < &u.user)
                    }).unwrap_or(top_3.len());
                    top_3.insert(insert_pos, new_user);
                }
            }
        }
        
        self.top_users = top_3;
    }
    
    /// Computes the 95th percentile of durations using the nearest-rank method
    /// rank = ceil(0.95 * n) (1-based), then p95 = d[rank - 1]
    /// Returns 0 if no events
    pub fn compute_p95_duration(&mut self, events: &[Event]) {
        if events.is_empty() {
            self.p95_duration_ms = 0;
            return;
        }
        
        // Extract and sort durations
        let mut durations: Vec<u64> = events.iter().map(|e| e.duration_ms).collect();
        durations.sort_unstable();
        
        // Calculate rank using nearest-rank method
        let n = durations.len();
        let rank = ((0.95 * n as f64).ceil() as usize).max(1);
        
        // Get p95 (convert from 1-based to 0-based index)
        self.p95_duration_ms = durations[rank - 1];
    }
    
    /// Finds the event with the maximum duration (outlier)
    pub fn compute_outlier(&mut self, events: &[Event]) {
        self.outlier = events.iter().max_by_key(|e| e.duration_ms).cloned();
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
    fn test_compute_top_users_basic() {
        let mut result = SummaryResult::new();
        let mut user_counts = HashMap::new();
        
        // alice: 5, bob: 3, carol: 1, dave: 7, eve: 2
        user_counts.insert("alice".to_string(), 5);
        user_counts.insert("bob".to_string(), 3);
        user_counts.insert("carol".to_string(), 1);
        user_counts.insert("dave".to_string(), 7);
        user_counts.insert("eve".to_string(), 2);
        
        result.compute_top_users(&user_counts);
        
        assert_eq!(result.top_users.len(), 3);
        
        // Top 3 should be: dave(7), alice(5), bob(3)
        assert_eq!(result.top_users[0].user, "dave");
        assert_eq!(result.top_users[0].count, 7);
        
        assert_eq!(result.top_users[1].user, "alice");
        assert_eq!(result.top_users[1].count, 5);
        
        assert_eq!(result.top_users[2].user, "bob");
        assert_eq!(result.top_users[2].count, 3);
    }
    
    #[test]
    fn test_compute_top_users_tie_breaker() {
        let mut result = SummaryResult::new();
        let mut user_counts = HashMap::new();
        
        // All have same count, should be sorted alphabetically
        user_counts.insert("charlie".to_string(), 5);
        user_counts.insert("alice".to_string(), 5);
        user_counts.insert("bob".to_string(), 5);
        user_counts.insert("dave".to_string(), 5);
        
        result.compute_top_users(&user_counts);
        
        assert_eq!(result.top_users.len(), 3);
        
        // Should be alphabetically: alice, bob, charlie (dave excluded)
        assert_eq!(result.top_users[0].user, "alice");
        assert_eq!(result.top_users[0].count, 5);
        
        assert_eq!(result.top_users[1].user, "bob");
        assert_eq!(result.top_users[1].count, 5);
        
        assert_eq!(result.top_users[2].user, "charlie");
        assert_eq!(result.top_users[2].count, 5);
    }
    
    #[test]
    fn test_compute_top_users_mixed_tie() {
        let mut result = SummaryResult::new();
        let mut user_counts = HashMap::new();
        
        // Mixed counts with ties
        user_counts.insert("alice".to_string(), 10);
        user_counts.insert("bob".to_string(), 5);
        user_counts.insert("carol".to_string(), 5);
        user_counts.insert("dave".to_string(), 3);
        
        result.compute_top_users(&user_counts);
        
        assert_eq!(result.top_users.len(), 3);
        
        // alice(10), bob(5), carol(5) - bob before carol alphabetically
        assert_eq!(result.top_users[0].user, "alice");
        assert_eq!(result.top_users[0].count, 10);
        
        assert_eq!(result.top_users[1].user, "bob");
        assert_eq!(result.top_users[1].count, 5);
        
        assert_eq!(result.top_users[2].user, "carol");
        assert_eq!(result.top_users[2].count, 5);
    }
    
    #[test]
    fn test_compute_top_users_less_than_three() {
        let mut result = SummaryResult::new();
        let mut user_counts = HashMap::new();
        
        user_counts.insert("alice".to_string(), 10);
        user_counts.insert("bob".to_string(), 5);
        
        result.compute_top_users(&user_counts);
        
        assert_eq!(result.top_users.len(), 2);
        assert_eq!(result.top_users[0].user, "alice");
        assert_eq!(result.top_users[1].user, "bob");
    }
    
    #[test]
    fn test_compute_top_users_empty() {
        let mut result = SummaryResult::new();
        let user_counts = HashMap::new();
        
        result.compute_top_users(&user_counts);
        
        assert_eq!(result.top_users.len(), 0);
    }
    
    #[test]
    fn test_compute_p95_single_event() {
        let mut result = SummaryResult::new();
        let events = vec![
            Event {
                ts: "2026-01-19T12:00:01Z".to_string(),
                level: Level::Info,
                user: "alice".to_string(),
                action: "test".to_string(),
                duration_ms: 100,
            }
        ];
        
        result.compute_p95_duration(&events);
        
        // n=1, rank=ceil(0.95*1)=1, p95=d[0]=100
        assert_eq!(result.p95_duration_ms, 100);
    }
    
    #[test]
    fn test_compute_p95_multiple_events() {
        let mut result = SummaryResult::new();
        let events = vec![
            Event {
                ts: "2026-01-19T12:00:01Z".to_string(),
                level: Level::Info,
                user: "alice".to_string(),
                action: "test".to_string(),
                duration_ms: 120,
            },
            Event {
                ts: "2026-01-19T12:00:02Z".to_string(),
                level: Level::Warn,
                user: "bob".to_string(),
                action: "test".to_string(),
                duration_ms: 400,
            },
            Event {
                ts: "2026-01-19T12:00:03Z".to_string(),
                level: Level::Error,
                user: "alice".to_string(),
                action: "test".to_string(),
                duration_ms: 900,
            },
            Event {
                ts: "2026-01-19T12:00:04Z".to_string(),
                level: Level::Info,
                user: "carol".to_string(),
                action: "test".to_string(),
                duration_ms: 20,
            },
            Event {
                ts: "2026-01-19T12:00:05Z".to_string(),
                level: Level::Info,
                user: "alice".to_string(),
                action: "test".to_string(),
                duration_ms: 10,
            },
        ];
        
        result.compute_p95_duration(&events);
        
        // Sorted: [10, 20, 120, 400, 900]
        // n=5, rank=ceil(0.95*5)=ceil(4.75)=5, p95=d[4]=900
        assert_eq!(result.p95_duration_ms, 900);
    }
    
    #[test]
    fn test_compute_p95_empty() {
        let mut result = SummaryResult::new();
        let events: Vec<Event> = vec![];
        
        result.compute_p95_duration(&events);
        
        assert_eq!(result.p95_duration_ms, 0);
    }
    
    #[test]
    fn test_compute_outlier() {
        let mut result = SummaryResult::new();
        let events = vec![
            Event {
                ts: "2026-01-19T12:00:01Z".to_string(),
                level: Level::Info,
                user: "alice".to_string(),
                action: "test".to_string(),
                duration_ms: 120,
            },
            Event {
                ts: "2026-01-19T12:00:02Z".to_string(),
                level: Level::Error,
                user: "bob".to_string(),
                action: "slow_task".to_string(),
                duration_ms: 900,
            },
            Event {
                ts: "2026-01-19T12:00:03Z".to_string(),
                level: Level::Info,
                user: "carol".to_string(),
                action: "test".to_string(),
                duration_ms: 50,
            },
        ];
        
        result.compute_outlier(&events);
        
        assert!(result.outlier.is_some());
        let outlier = result.outlier.unwrap();
        assert_eq!(outlier.duration_ms, 900);
        assert_eq!(outlier.user, "bob");
        assert_eq!(outlier.action, "slow_task");
    }
    
    #[test]
    fn test_compute_outlier_empty() {
        let mut result = SummaryResult::new();
        let events: Vec<Event> = vec![];
        
        result.compute_outlier(&events);
        
        assert!(result.outlier.is_none());
    }
}


