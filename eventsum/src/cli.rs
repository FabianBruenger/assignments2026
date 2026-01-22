use clap::Parser;

#[derive(Parser)]
#[command(name = "eventsum")]
#[command(about = "Parses event log (JSON Lines) and produces a summary report")]
#[command(long_about = "Parses event log (JSON Lines) and produces a summary report.\n\nLogging:\n  Set RUST_LOG environment variable to control log output:\n  - RUST_LOG=error  : Errors only\n  - RUST_LOG=info   : Major operations\n  - RUST_LOG=debug  : Detailed line processing\n  - RUST_LOG=trace  : Maximum verbosity\n\nExample:\n  RUST_LOG=info eventsum --input events.jsonl")]
pub struct Cli {
    /// Input file path. If omitted, reads from stdin
    #[arg(short, long)]
    pub input: Option<String>,

    /// Pretty-print the output JSON
    #[arg(long)]
    pub pretty: bool,
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::io::Write;
    use std::fs::File;

    #[test]
    fn test_help_flag() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("eventsum"));
        assert!(stdout.contains("--input"));
    }

    #[test]
    fn test_file_not_found() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--input", "nonexistent_file.json"])
            .output()
            .expect("Failed to execute command");
        
        // Should exit with code 2
        assert_eq!(output.status.code(), Some(2));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Error reading input"));
    }

    #[test]
    fn test_read_from_file() {
        // Create a temporary test file
        let test_file = "/tmp/eventsum_test.jsonl";
        let mut file = File::create(test_file).expect("Failed to create test file");
        writeln!(file, r#"{{"ts":"2026-01-19T12:00:01Z","level":"INFO","user":"alice","action":"test","duration_ms":100}}"#)
            .expect("Failed to write test data");
        writeln!(file, "").expect("Failed to write blank line");
        writeln!(file, r#"{{"ts":"2026-01-19T12:00:02Z","level":"WARN","user":"bob","action":"test2","duration_ms":200}}"#)
            .expect("Failed to write test data");
        drop(file);
        
        let output = Command::new("cargo")
            .args(&["run", "--", "--input", test_file])
            .output()
            .expect("Failed to execute command");
        
        // Should exit with code 0
        assert_eq!(output.status.code(), Some(0));
        
        // Clean up
        std::fs::remove_file(test_file).ok();
    }

    #[test]
    fn test_read_from_stdin() {
        let mut child = Command::new("cargo")
            .args(&["run"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn command");
        
        {
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin.write_all(br#"{"ts":"2026-01-19T12:00:01Z","level":"INFO","user":"alice","action":"test","duration_ms":100}"#)
                .expect("Failed to write to stdin");
            stdin.write_all(b"\n").expect("Failed to write newline");
        }
        
        let output = child.wait_with_output().expect("Failed to wait for command");
        
        // Should exit with code 0
        assert_eq!(output.status.code(), Some(0));
    }
}