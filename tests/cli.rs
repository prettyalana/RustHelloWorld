use assert_cmd::Command;

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
    cmd.assert().success();
}