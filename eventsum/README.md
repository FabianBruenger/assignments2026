# Eventsum

Main crates:

* clap for command line: https://crates.io/crates/clap

## Run

Build for your local target architecture.

```bash
# Build
cargo build --release
# Run
./target/release/eventsum --help
./target/release/eventsum --input test.json
```

## Test

## Notes:

1. Event struct -> has the specific fields
2. Result struct -> Holds the field of the results
3. app -> holds vector of even struct
4. app -> method 1: read in file line by line and ignore blanks. Fail with 2 if error
5. app -> method 2: read in from stdin if flag is not set (blanks possible?). Fail with 2 if error
6. app -> method 3: validate. If bad line, increase counter, if not process