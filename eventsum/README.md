# Eventsum

Showcase Rust project - Build a small Rust command-line tool that parses a simple event log (JSON Lines) and produces a compact summary report.

## Run

Build for your local target architecture.

```bash
# Build
cargo build --release
# Run
./target/release/eventsum --help
./target/release/eventsum --input test.jsonl 
./target/release/eventsum --input test.jsonl --pretty
RUST_LOG=debug ./target/release/eventsum --input test.jsonl --pretty

cat mock_data/test.jsonl | RUST_LOG=debug ./target/release/eventsum --pretty
```

## Test

Run unit tests with:
```
cargo test
```