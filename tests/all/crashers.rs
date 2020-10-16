use crate::helpers::{command, get_corpus_path};

#[test]
fn does_not_crash() {
    let crashers_dir = std::fs::read_dir(get_corpus_path().join("old-crashers")).unwrap();
    for crasher in crashers_dir {
        let crasher = crasher.unwrap();

        let mut cmd = command();
        cmd.pipe_stdin(crasher.path()).unwrap();
        cmd.assert().success();
    }
}
