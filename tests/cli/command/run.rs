use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;



#[test]
fn invalid_bytecode_file() {
    let temp_dir = TempDir::new().unwrap();
    let invalid_file = temp_dir.path().join("invalid.sip");
    
    fs::write(&invalid_file, "not a valid bytecode file").unwrap();
    
    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("run")
        .arg(&invalid_file)
        .assert()
        .failure()
        .code(65); // Dataerr exit code
}

#[test]
fn non_existent_file() {
    let temp_dir = TempDir::new().unwrap();
    let non_existent_file = temp_dir.path().join("non_existent.sip");   

    let mut cmd = Command::cargo_bin("slang").unwrap();
    cmd.arg("run")
        .arg(&non_existent_file)
        .assert()
        .failure()
        .code(66); // No such file or directory exit code
}
