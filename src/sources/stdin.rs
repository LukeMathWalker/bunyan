use crate::record::LogRecord;
use std::io::BufRead;

pub fn process_stdin() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("{}", line);
        match serde_json::from_str::<LogRecord>(&line) {
            Ok(_) => println!("Parsed!"),
            Err(e) => println!("Failed to parse - {:?}", e),
        }
    }
}
