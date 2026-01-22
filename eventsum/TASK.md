# Eventsum - Rust assignment

Max working time spend: 1h.

## Goal
Build a small Rust command-line tool that parses a simple event log (JSON Lines) and produces a compact summary report.

## Problem Statement
You are given an input file containing JSON Lines (one JSON object per line). Each line represents an event produced by a system.
Event format
Each line is a JSON object with the following fields:

```
ts (string): ISO-8601 timestamp (UTC), e.g. "2026-01-19T12:34:56Z"
level (string): one of INFO, WARN, ERROR
user (string): username (non-empty)
action (string): name of the action (non-empty)
duration_ms (integer): duration in milliseconds (>= 0)
````

Example line:

```json
{"ts":"2026-01-19T12:00:01Z","level":"INFO","user":"alice","action":"run_script","duration_ms":120}
```

## Task
Implement a CLI tool called *eventsum* that:

* Reads events from a file (--input <path>) or from stdin if --input is omitted.
* Parses JSON Lines and ignores blank lines.
* Validates events:
    * If a line is invalid JSON or missing required fields, it should be treated as a bad line.
    * Bad lines should not crash the program.
* Computes the following summary:
    * Required output (JSON)
* Print a single JSON object to stdout with:

```
total_lines (integer): total non-blank lines read
bad_lines (integer): count of lines that were invalid
events (integer): count of valid events
by_level (object): counts per level (keys: INFO, WARN, ERROR)
top_users (array): the top 3 users by number of valid events, descending by count, tie-breaker by username ascending.
Each element: { "user": "<name>", "count": <int> }
p95_duration_ms (integer): the 95th percentile of duration_ms across valid events (see percentile definition below). If there are no valid events, output 0
outlier (object): copy of an event with the largest duration_ms.
```

## Percentile definition:

Let d be the sorted list of durations (ascending) of length n.
Use the nearest-rank method:
rank = ceil(0.95 * n) (1-based), then p95 = d[rank - 1].
Example: if n = 1, rank = 1, so p95 = d[0].


## CLI Requirements
eventsum --input path/to/file.json
cat file.jsonl | eventsum
Optional: --pretty to pretty-print the output JSON (indentation).

Exit code:

0 on success (even if some bad lines exist).
2 if input cannot be read (file not found, permission denied, etc.).

### Example Input

```json
{"ts":"2026-01-19T12:00:01Z","level":"INFO","user":"alice","action":"run_script","duration_ms":120}
{"ts":"2026-01-19T12:00:02Z","level":"WARN","user":"bob","action":"upload_data","duration_ms":400}
not-json
{"ts":"2026-01-19T12:00:04Z","level":"ERROR","user":"alice","action":"run_script","duration_ms":900}
{"ts":"2026-01-19T12:00:05Z","level":"INFO","user":"carol","action":"login","duration_ms":20}
{"ts":"2026-01-19T12:00:06Z","level":"INFO","user":"alice","action":"logout","duration_ms":10}
```

### Example Output (pretty)
```json
{
  "total_lines": 6,
  "bad_lines": 1,
  "events": 5,
  "by_level": { "INFO": 3, "WARN": 1, "ERROR": 1 },
  "top_users": [
    { "user": "alice", "count": 3 },
    { "user": "bob", "count": 1 },
    { "user": "carol", "count": 1 }
  ],
  "p95_duration_ms": 900,
  "outlier": {
    "ts": "2026-01-19T12:00:04Z",
    "level": "ERROR",
    "user": "alice",
    "action": "run_script",
    "duration_ms": 900
  }
}
```


## Constraints / Expectations
Keep the solution small and readable.
Prefer idiomatic Rust and reasonable error handling.
You may use common crates
You do not need to fully parse ts into a datetime; treating it as a string is acceptable for this task.


# What to Submit
A git repository or a zip containing:
Source code
Cargo.toml
A short README.md with:
build instructions
how to run on the example input
any tradeoffs or assumptions


Evaluation Criteria
Correctness (matches requirements)
Robustness (doesn’t crash on bad lines)
Code clarity and structure
Reasonable CLI UX (help text, error messages, exit codes)


Optional Stretch (if you have time)
Pick one (not required):

Add --min-level <INFO|WARN|ERROR> to include only events at or above the given severity.
Add --top <N> to control the number of top users.
Add unit tests for percentile and top-user ordering.
