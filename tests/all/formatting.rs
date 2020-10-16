use crate::helpers::{command, get_corpus_path};
use predicates::prelude::predicate;

#[test]
fn simple_log() {
    let input_path = get_corpus_path().join("simple.log");

    let mut cmd = command();
    cmd.pipe_stdin(input_path).unwrap();
    cmd.assert().success().stdout(predicate::str::similar(
        "[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message\n",
    ));
}
