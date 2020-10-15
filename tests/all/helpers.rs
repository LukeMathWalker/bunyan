use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

pub fn command() -> Command {
    Command::cargo_bin("bunyan").unwrap()
}
