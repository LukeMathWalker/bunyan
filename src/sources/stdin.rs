use crate::record::LogRecord;
use crate::Format;
use std::io::BufRead;

pub fn process_stdin(format: Format, level_filter: u8, strict: bool) {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match serde_json::from_str::<LogRecord>(&line) {
            Ok(r) => {
                if r.level >= level_filter {
                    print!("{}", r.format(format))
                }
            }
            Err(_) => {
                if !strict {
                    println!("{}", line)
                }
            }
        }
    }
}
