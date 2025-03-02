use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test] // unit test function marker 
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap(); // unwrap will cause a panic and the test will fail
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, "Hello, world!\n");
}