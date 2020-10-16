use bunyan::{process_stdin, Format, NumericalLogLevel};
use clap::Clap;

/// Filter and pretty-print Bunyan log file content.
#[derive(Clap)]
#[clap(version = "0.1", author = "Luca Palmieri <rust@lpalmieri.com>")]
struct Cli {
    /// Only show messages at or above the specified level.
    ///
    /// You can specify level names (trace, debug, info, warn, error, fatal) or a positive
    /// numeric value.
    #[clap(short, long, default_value = "trace")]
    level: NumericalLogLevel,
    /// Specify an output format.
    ///
    /// - long: prettified JSON;
    #[clap(short, long, default_value = "long")]
    output: Format,
}

fn main() {
    let cli = Cli::parse();
    process_stdin(cli.output);
}
