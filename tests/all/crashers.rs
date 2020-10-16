use crate::helpers::{command, get_corpus_path};

#[test]
fn does_not_crash() {
    let crasher_path = get_corpus_path().join("old-crashers").join("139.log");

    let mut cmd = command();
    cmd.pipe_stdin(crasher_path).unwrap();
    cmd.assert().success();
}
