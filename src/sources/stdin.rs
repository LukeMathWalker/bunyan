use crate::record::LogRecord;
use crate::Format;
use quick_js::Context;
use std::io::BufRead;

pub fn process_stdin(format: Format, level_filter: u8, condition: Option<String>, strict: bool) {
    let stdin = std::io::stdin();
    let context = Context::new().unwrap();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match serde_json::from_str::<LogRecord>(&line) {
            Ok(r) => {
                if r.level < level_filter {
                    continue;
                }
                if let Some(condition) = &condition {
                    let condition = context
                        .eval_as::<bool>(
                            format!("(function (){{return ({condition})}}).call({line})").as_str(),
                        )
                        .unwrap();
                    if !condition {
                        continue;
                    }
                }
                print!("{}", r.format(format))
            }
            Err(_) => {
                if !strict {
                    println!("{line}")
                }
            }
        }
    }
}
