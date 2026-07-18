use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

fn test_bin() -> Command {
    let mut cmd = Command::cargo_bin("hashcheck").unwrap();
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd
}

#[test]
fn test_hash_file() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let filepath = tmpdir.path().join("test.txt");
    fs::write(&filepath, "hello world").unwrap();

    let cmd = test_bin().arg("hash").arg(&filepath).assert().success();
    let output = cmd.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"));
}

#[test]
fn test_hash_stdin() {
    let cmd = test_bin()
        .arg("hash")
        .arg("-")
        .write_stdin("hello world")
        .assert()
        .success();
    let output = cmd.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"));
}

#[test]
fn test_hash_sha512() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let filepath = tmpdir.path().join("test.txt");
    fs::write(&filepath, "hello").unwrap();

    test_bin()
        .arg("hash")
        .arg(&filepath)
        .arg("--algo")
        .arg("sha512")
        .assert()
        .success();

    let cmd = test_bin()
        .arg("hash")
        .arg(&filepath)
        .arg("--algo")
        .arg("sha512")
        .assert()
        .success();
    let output = cmd.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"));
}

#[test]
fn test_hash_md5() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let filepath = tmpdir.path().join("test.txt");
    fs::write(&filepath, "hello").unwrap();

    let cmd = test_bin()
        .arg("hash")
        .arg(&filepath)
        .arg("--algo")
        .arg("md5")
        .assert()
        .success();
    let output = cmd.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn test_compare_match() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let f1 = tmpdir.path().join("a.txt");
    let f2 = tmpdir.path().join("b.txt");
    fs::write(&f1, "same content").unwrap();
    fs::write(&f2, "same content").unwrap();

    test_bin()
        .arg("compare")
        .arg(&f1)
        .arg(&f2)
        .assert()
        .success()
        .stdout(predicate::str::contains("MATCH"));
}

#[test]
fn test_compare_different() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let f1 = tmpdir.path().join("a.txt");
    let f2 = tmpdir.path().join("b.txt");
    fs::write(&f1, "content a").unwrap();
    fs::write(&f2, "content b").unwrap();

    test_bin()
        .arg("compare")
        .arg(&f1)
        .arg(&f2)
        .assert()
        .failure()
        .stdout(predicate::str::contains("DIFFER"));
}

#[test]
fn test_json_output() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let filepath = tmpdir.path().join("test.txt");
    fs::write(&filepath, "test").unwrap();

    let cmd = test_bin()
        .arg("hash")
        .arg(&filepath)
        .arg("-f")
        .arg("json")
        .assert()
        .success();
    let output = cmd.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"algorithm\""));
    assert!(stdout.contains("\"hash\""));
    assert!(stdout.contains("\"file\""));
}

#[test]
fn test_csv_output() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let filepath = tmpdir.path().join("test.txt");
    fs::write(&filepath, "test").unwrap();

    let cmd = test_bin()
        .arg("hash")
        .arg(&filepath)
        .arg("-f")
        .arg("csv")
        .assert()
        .success();
    let output = cmd.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("algorithm"));
    assert!(stdout.contains("hash"));
    assert!(stdout.contains("file"));
}

#[test]
fn test_dir_hash() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let f1 = tmpdir.path().join("a.txt");
    let f2 = tmpdir.path().join("b.txt");
    fs::write(&f1, "aaa").unwrap();
    fs::write(&f2, "bbb").unwrap();

    let cmd = test_bin().arg("dir").arg(tmpdir.path()).assert().success();
    let output = cmd.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("a.txt"));
    assert!(stdout.contains("b.txt"));
}

#[test]
fn test_genfile() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let out = tmpdir.path().join("checksums.txt");
    let f1 = tmpdir.path().join("a.txt");
    let f2 = tmpdir.path().join("b.txt");
    fs::write(&f1, "aaa").unwrap();
    fs::write(&f2, "bbb").unwrap();

    test_bin()
        .arg("genfile")
        .arg(tmpdir.path())
        .arg("-o")
        .arg(&out)
        .assert()
        .success();

    assert!(out.exists());
    let content = fs::read_to_string(&out).unwrap();
    assert!(content.contains("a.txt"));
    assert!(content.contains("b.txt"));
}

#[test]
fn test_genfile_verify() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let f1 = tmpdir.path().join("a.txt");
    let f2 = tmpdir.path().join("b.txt");
    fs::write(&f1, "aaa").unwrap();
    fs::write(&f2, "bbb").unwrap();

    // Generate checksums to checksums.txt in the same directory
    let out = tmpdir.path().join("checksums.txt");
    test_bin()
        .arg("genfile")
        .arg(tmpdir.path())
        .arg("-o")
        .arg(&out)
        .assert()
        .success();

    // Verify against base directory
    test_bin()
        .arg("verify")
        .arg(&out)
        .arg("--base")
        .arg(tmpdir.path())
        .assert()
        .success();
}

#[test]
fn test_verify_modified_file() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let f1 = tmpdir.path().join("a.txt");
    fs::write(&f1, "original").unwrap();

    // Generate checksums
    let _cmd = test_bin()
        .arg("genfile")
        .arg(tmpdir.path())
        .output()
        .unwrap();

    let checksum_content = format!(
        "{}  a.txt\n",
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
    let checksum_path = tmpdir.path().join("checksums.txt");
    fs::write(&checksum_path, checksum_content).unwrap();

    // Modify the file
    fs::write(&f1, "modified").unwrap();

    // Verify should fail
    test_bin()
        .arg("verify")
        .arg(&checksum_path)
        .arg("--base")
        .arg(tmpdir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("FAILED"));
}

#[test]
fn test_verify_missing_file() {
    let tmpdir = tempfile::Builder::new().prefix("tmp").tempdir().unwrap();
    let checksum_content =
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9  nonexistent.txt\n";
    let checksum_path = tmpdir.path().join("checksums.txt");
    fs::write(&checksum_path, checksum_content).unwrap();

    test_bin()
        .arg("verify")
        .arg(&checksum_path)
        .arg("--base")
        .arg(tmpdir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("MISSING"));
}

#[test]
fn test_version() {
    test_bin().arg("--version").assert().success();
}

#[test]
fn test_help() {
    test_bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Checksum toolkit"));
}
