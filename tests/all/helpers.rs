use assert_cmd::Command;
use std::path::PathBuf;

pub fn command() -> Command {
    Command::cargo_bin("bunyan").unwrap()
}

pub fn get_corpus_path() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::path::PathBuf::from(manifest_dir)
        .join("tests")
        .join("all")
        .join("corpus")
}
