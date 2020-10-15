use std::io::BufRead;

pub fn process_stdin() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
