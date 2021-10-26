use crate::helpers::{command, get_corpus_path};
use predicates::prelude::predicate;

#[test]
fn invalid_comparison_operators() {
    let test_file = get_corpus_path().join("log1.log");

    for invalid_operator in &["key=value", "key*value"] {
        let mut cmd = command();

        cmd.arg("-c")
            .arg(invalid_operator)
            .pipe_stdin(&test_file)
            .unwrap();
        cmd.assert().failure();
    }
}

#[test]
fn valid_comparison_operators() {
    let test_file = get_corpus_path().join("log1.log");
    for valid_operator in &[
        "key==value",
        "key!=value",
        "key<value",
        "key>value",
        "key <= value",
        "key >= value",
    ] {
        let mut cmd = command();

        cmd.arg("-c")
            .arg(valid_operator)
            .pipe_stdin(&test_file)
            .unwrap();
        cmd.assert().success();
    }
}

#[test]
fn compare_version() {
    let test_file = get_corpus_path().join("log1.log");
    let expected_result = "[2012-05-08T16:57:55.586Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.339Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.404Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.404Z]  INFO: agent1/73267 on headnode: message
";
    let mut cmd = command();

    cmd.arg("-c")
        .arg("version == 0")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_version_does_not_match() {
    let test_file = get_corpus_path().join("log1.log");

    let mut cmd = command();

    cmd.arg("-c")
        .arg("version == 1")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert().success().stdout(predicate::str::diff(""));
}
#[test]
fn compare_pid() {
    let test_file = get_corpus_path().join("log2.log");
    let expected_result = "[2012-05-08T16:58:55.586Z]  INFO: agent2/73267 on headnode: message
[2012-05-08T17:01:49.339Z]  INFO: agent2/73267 on headnode: message
[2012-05-08T17:02:47.404Z]  INFO: agent2/73267 on headnode: message
[2012-05-08T17:02:57.404Z]  INFO: agent2/73267 on headnode: message
";
    let mut cmd = command();

    cmd.arg("-c")
        .arg("pid == 73267")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_pid_does_not_match() {
    let test_file = get_corpus_path().join("log2.log");

    let mut cmd = command();

    cmd.arg("-c")
        .arg("pid < 50")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert().success().stdout(predicate::str::diff(""));
}

#[test]
fn compare_level() {
    let test_file = get_corpus_path().join("log1.log");
    let expected_result = "[2012-05-08T16:57:55.586Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.339Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.404Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.404Z]  INFO: agent1/73267 on headnode: message
";
    let mut cmd = command();

    cmd.arg("-c")
        .arg("level == 30")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_level_does_not_match() {
    let test_file = get_corpus_path().join("log1.log");

    let mut cmd = command();

    cmd.arg("-c")
        .arg("level >= 35")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert().success().stdout(predicate::str::diff(""));
}
#[test]
fn compare_name() {
    let test_file = get_corpus_path().join("log1.log");
    let expected_result = "[2012-05-08T16:57:55.586Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.339Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.404Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.404Z]  INFO: agent1/73267 on headnode: message
";
    let mut cmd = command();

    cmd.arg("-c")
        .arg("name == agent1")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_name_does_not_match() {
    let test_file = get_corpus_path().join("log1.log");

    let mut cmd = command();

    cmd.arg("-c")
        .arg("name == fred")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert().success().stdout(predicate::str::diff(""));
}
#[test]
fn compare_hostname() {
    let test_file = get_corpus_path().join("log1.log");
    let expected_result = "[2012-05-08T16:57:55.586Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.339Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.404Z]  INFO: agent1/73267 on headnode: message
[2012-05-08T17:02:49.404Z]  INFO: agent1/73267 on headnode: message
";
    let mut cmd = command();

    cmd.arg("-c")
        .arg("hostname == headnode")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_hostname_does_not_match() {
    let test_file = get_corpus_path().join("log1.log");

    let mut cmd = command();

    cmd.arg("-c")
        .arg("hostname == 3")
        .arg("--no-color")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert().success().stdout(predicate::str::diff(""));
}

#[test]
fn compare_in_extra() {
    let test_file = get_corpus_path().join("all.log");
    let expected_result =
        "[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message (one=short)\n";

    let mut cmd = command();

    cmd.arg("-c")
        .arg("one==short")
        .arg("--no-color")
        .arg("--strict")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_longer_phrase_in_extra() {
    let test_file = get_corpus_path().join("all.log");
    let expected_result =
        "[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message (two=\"short with space\")\n";

    let mut cmd = command();

    cmd.arg("-c")
        .arg("two==short with space")
        .arg("--no-color")
        .arg("--strict")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_embedded_object() {
    let test_file = get_corpus_path().join("all.log");
    let expected_result = r#"[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    five: {
      "a": "json object"
    }
"#;

    let mut cmd = command();

    cmd.arg("-c")
        .arg("a==json object")
        // .arg("pid==123")
        .arg("--no-color")
        .arg("--strict")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_double_embedded_object() {
    let test_file = get_corpus_path().join("all.log");
    let expected_result = r#"[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    seven: {
      "a": "data",
      "b": {
        "c": "double-nested1",
        "d": 12.5
      }
    }
"#;

    let mut cmd = command();

    cmd.arg("-c")
        .arg("c==double-nested1")
        // .arg("pid==123")
        .arg("--no-color")
        .arg("--strict")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}

#[test]
fn compare_floating_point_numbers() {
    let test_file = get_corpus_path().join("all.log");
    let expected_result = r#"[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    seven: {
      "a": "data",
      "b": {
        "c": "double-nested1",
        "d": 12.5
      }
    }
"#;

    let mut cmd = command();

    cmd.arg("-c")
        .arg("d==12.5")
        // .arg("pid==123")
        .arg("--no-color")
        .arg("--strict")
        .pipe_stdin(&test_file)
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected_result));
}
