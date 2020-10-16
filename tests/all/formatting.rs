use crate::helpers::{command, get_corpus_path};
use predicates::prelude::predicate;

#[test]
fn simple_log() {
    let input_path = get_corpus_path().join("simple.log");

    let mut cmd = command();
    cmd.arg("--no-color").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicate::str::similar(
        "[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message\n",
    ));
}

#[test]
fn simple_log_with_color() {
    let input_path = get_corpus_path().join("simple.log");

    let mut cmd = command();
    cmd.arg("--color").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicate::str::similar(
        "[2012-02-08T22:56:52.856Z]  \u{1b}[36mINFO\u{1b}[0m: myservice/123 on example.com: \u{1b}[36mMy message\u{1b}[0m\n",
    ));
}

#[test]
fn extra_field_log() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("--no-color").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicate::str::similar(
        "[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message (extra=field)\n",
    ));
}
