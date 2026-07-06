use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_sample() {
    let input = b"6
INSERT 10
INSERT 25
FIND 10
FIND 7
INSERT 7
FIND 7
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "b-tree"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run b-tree");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(
        stdout.trim(),
        "FOUND
NOT FOUND
FOUND"
    );
}

#[test]
fn test_all_found() {
    let input = b"5
INSERT 1
INSERT 2
FIND 1
FIND 2
FIND 1
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "b-tree"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run b-tree");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(
        stdout.trim(),
        "FOUND
FOUND
FOUND"
    );
}

#[test]
fn test_all_missing() {
    let input = b"3
FIND 10
FIND 20
FIND 30
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "b-tree"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run b-tree");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(
        stdout.trim(),
        "NOT FOUND
NOT FOUND
NOT FOUND"
    );
}