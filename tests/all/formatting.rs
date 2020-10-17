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
        "[2012-02-08T22:56:52.856Z] \u{1b}[36m INFO\u{1b}[0m: myservice/123 on example.com: \u{1b}[36mMy message\u{1b}[0m\n",
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

#[test]
fn extra_field_log_with_color() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("--color").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicate::str::similar(
        "[2012-02-08T22:56:52.856Z] \u{1b}[36m INFO\u{1b}[0m: myservice/123 on example.com: \u{1b}[36mMy message\u{1b}[0m (extra=field)\n",
    ));
}

#[test]
fn bogus_log() {
    let input_path = get_corpus_path().join("bogus.log");

    let mut cmd = command();
    cmd.arg("--no-color").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicate::str::similar(
        r#"not a JSON line
{"hi": "there"}
"#,
    ));
}

#[test]
fn bogus_log_strict() {
    let input_path = get_corpus_path().join("bogus.log");

    let mut cmd = command();
    cmd.arg("--no-color")
        .arg("--strict")
        .pipe_stdin(input_path)
        .unwrap();
    cmd.assert().success().stdout(predicate::str::similar(""));
}

#[test]
fn all_works() {
    let input_path = get_corpus_path().join("all.log");

    let mut cmd = command();
    cmd.arg("--no-color").pipe_stdin(input_path).unwrap();
    cmd.assert().success();
}

#[test]
fn level_filter() {
    let input_path = get_corpus_path().join("all.log");
    let expected = "# levels
[2012-02-08T22:56:53.856Z]  WARN: myservice/123 on example.com: My message
[2012-02-08T22:56:54.856Z] ERROR: myservice/123 on example.com: My message
[2012-02-08T22:56:55.856Z] LVL55: myservice/123 on example.com: My message
[2012-02-08T22:56:56.856Z] FATAL: myservice/123 on example.com: My message

# extra fields

# bogus
not a JSON line
{\"hi\": \"there\"}\n";

    let mut cmd = command();
    cmd.arg("--no-color")
        .arg("-l")
        .arg("40")
        .pipe_stdin(input_path)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(expected));
}
