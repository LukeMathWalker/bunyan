use bunyan::{process_stdin, LogLevel};
use clap::Clap;

/// Filter and pretty-print Bunyan log file content.
#[derive(Clap)]
#[clap(version = "0.1", author = "Luca Palmieri <rust@lpalmieri.com>")]
struct Cli {
    /// Only show messages at or above the specified level.
    /// You can specify level *names* or a positive numeric value.
    #[clap(short, long, default_value = "trace")]
    level: LogLevel,
}

fn main() {
    let _cli = Cli::parse();
    process_stdin();
}
