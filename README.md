# assignments2026

Rust Assignment - Event Log Parser

## Overview

This repository contains a Rust command-line tool that parses JSON Lines event logs and produces compact summary reports.

## Features

- Parses JSON Lines format (one JSON object per line)
- Generates summary statistics for event logs
- Counts total events
- Groups events by type and severity level
- Handles flexible JSON structures with optional fields
- Robust error handling

## Building

```bash
cargo build --release
```

## Usage

```bash
./target/release/event_log_parser <event_log_file>
```

### Example

```bash
./target/release/event_log_parser sample_events.jsonl
```

### Sample Output

```
=== Event Log Summary Report ===

Total Events: 10

Events by Type:
  user_login: 3
  api_call: 3
  error: 2
  user_logout: 1
  warning: 1

Events by Level:
  info: 7
  error: 2
  warning: 1

================================
```

## Event Log Format

The tool expects a JSON Lines file where each line is a valid JSON object representing an event. Events can have the following fields (all optional):

- `type`: The type/category of the event
- `timestamp`: ISO 8601 timestamp
- `level`: Severity level (e.g., info, warning, error)
- `message`: Human-readable message
- Additional custom fields are supported

### Example Event

```json
{"type":"user_login","timestamp":"2026-01-22T07:00:00Z","level":"info","message":"User logged in successfully","user_id":123}
```

## Testing

A sample event log file (`sample_events.jsonl`) is included for testing purposes.

## Requirements

- Rust 1.70 or later
- Dependencies are managed via Cargo
