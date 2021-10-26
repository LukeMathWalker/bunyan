use crate::compare::compare_operators::do_compare;
use crate::compare::compare_operators::CompiledComparison;
use crate::record::LogRecord;
use crate::Format;
use std::io::BufRead;

pub fn process_stdin(format: Format, level_filter: u8, strict: bool, compare: Option<String>) {
    let stdin = std::io::stdin();
    let compare_set;
    let cc;

    match compare {
        Some(c_string) => {
            compare_set = true;
            let compare_string = c_string;
            cc = Some(CompiledComparison::new(&compare_string));
        }
        None => {
            compare_set = false;
            cc = None;
        }
    }
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match serde_json::from_str::<LogRecord>(&line) {
            Ok(r) => {
                if r.level >= level_filter {
                    if compare_set {
                        if do_compare(&r, cc.as_ref().unwrap()) {
                            print!("{}", r.format(format));
                        }
                    } else {
                        print!("{}", r.format(format))
                    }
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
