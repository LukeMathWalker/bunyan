use bunyan::{process_stdin, Format, NumericalLogLevel};
use clap::Parser;

/// Bunyan is a simple and fast JSON logging library for node.js services,
/// a one-JSON-object-per-line log format, and a bunyan CLI tool for nicely viewing those logs.
/// this is a Rust implementation of bunyan cli used to filter and pretty-print Bunyan log file content.
#[derive(clap::StructOpt)]
#[clap(version = "0.1", author = "Luca Palmieri <rust@lpalmieri.com>")]
struct Cli {
    /// Only show messages at or above the specified level.
    ///
    /// You can specify level names (trace, debug, info, warn, error, fatal) or a positive
    /// numeric value.
    #[clap(short, long, default_value = "trace")]
    level: NumericalLogLevel,
    /// Run each log message through the condition and only show those that return truish.
    /// E.g.:
    ///     -c 'this.pid == 123'
    ///     -c 'this.level == DEBUG'
    ///     -c 'this.msg.indexOf("boom") != -1'
    /// "CONDITION" must be (somewhat) legal JS code.
    /// `this` holds the log record.
    /// The TRACE, DEBUG, ... FATAL values are defined to help with comparing `this.level`.
    #[clap(short, long)]
    condition: Option<String>,
    /// Specify an output format.
    ///
    /// - long: prettified JSON;
    #[clap(short, long, default_value = "long")]
    output: Format,
    /// Colorize output.
    ///
    /// Defaults to try if output stream is a TTY.
    #[clap(long = "color", conflicts_with = "no-color")]
    color: bool,
    /// Force no coloring (e.g. terminal doesn't support it).
    #[clap(long = "no-color", conflicts_with = "color")]
    no_color: bool,
    /// Suppress all but legal Bunyan JSON log lines. By default non-JSON and non-Bunyan lines
    /// are passed through.
    #[clap(long)]
    strict: bool,
}

fn main() {
    let cli = Cli::parse();

    // Color output if explicitly requested or if the terminal supports it, unless the user
    // explicitly opted out.
    if cli.no_color {
        colored::control::set_override(false);
    } else if cli.color || atty::is(atty::Stream::Stdout) {
        colored::control::set_override(true);
    }

    process_stdin(cli.output, cli.level.0, cli.condition, cli.strict);
}
