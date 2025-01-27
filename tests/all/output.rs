use crate::helpers::{command, get_corpus_path};

#[test]
fn extra_field_long() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("-o").arg("long").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicates::str::diff(
        "[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message (extra=field)\n",
    ));
}

#[test]
fn extra_field_short() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("-o").arg("short").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicates::str::diff(
        "22:56:52.856Z  INFO myservice: My message (extra=field)\n",
    ));
}

#[test]
fn extra_field_json() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("-o").arg("json").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicates::str::diff(
        r#"{
  "v": 0,
  "name": "myservice",
  "msg": "My message",
  "level": 30,
  "hostname": "example.com",
  "pid": 123,
  "time": "2012-02-08T22:56:52.856Z",
  "extra": "field"
}"#,
    ));
}

#[test]
fn extra_field_json4() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("-o").arg("json-4").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicates::str::diff(
        r#"{
    "v": 0,
    "name": "myservice",
    "msg": "My message",
    "level": 30,
    "hostname": "example.com",
    "pid": 123,
    "time": "2012-02-08T22:56:52.856Z",
    "extra": "field"
}"#,
    ));
}

#[test]
fn extra_field_json_more_than_10_still_10() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("-o").arg("json-25").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicates::str::diff(
        r#"{
          "v": 0,
          "name": "myservice",
          "msg": "My message",
          "level": 30,
          "hostname": "example.com",
          "pid": 123,
          "time": "2012-02-08T22:56:52.856Z",
          "extra": "field"
}"#,
    ));
}

#[test]
fn extra_field_json_0() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("-o").arg("json-0").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicates::str::diff(r#"{"v":0,"name":"myservice","msg":"My message","level":30,"hostname":"example.com","pid":123,"time":"2012-02-08T22:56:52.856Z","extra":"field"}
"#
    ));
}

#[test]
fn extra_field_bunyan() {
    let input_path = get_corpus_path().join("extrafield.log");

    let mut cmd = command();
    cmd.arg("-o").arg("bunyan").pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicates::str::diff(r#"{"v":0,"name":"myservice","msg":"My message","level":30,"hostname":"example.com","pid":123,"time":"2012-02-08T22:56:52.856Z","extra":"field"}
"#
    ));
}
