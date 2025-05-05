use std::process::Command;

#[test]
fn test_demo() {
    println!("demo test");
}

#[test]
fn test_command() {
    let output = Command::new("java")
        .arg("-version")
        .output()
        .expect("Failed to execute command");
    String::from_utf8(output.stderr).unwrap();
}
