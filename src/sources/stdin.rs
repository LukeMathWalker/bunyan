use crate::record::LogRecord;
use std::io::BufRead;

pub fn process_stdin() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match serde_json::from_str::<LogRecord>(&line) {
            Ok(r) => println!(
                "[{}] {}: {}/{} on {}: {}",
                r.time, r.level, r.name, r.process_identifier, r.hostname, r.message
            ),
            Err(e) => eprintln!("Failed to parse - {:?}", e),
        }
    }
}
