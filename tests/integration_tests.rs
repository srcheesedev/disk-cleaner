//! Integration tests for the disk cleaner application
//!
//! These tests verify end-to-end functionality including CLI argument parsing,
//! directory analysis, and cross-module interactions.
//!
//! Copyright (c) 2025 @srcheesedev
//! Licensed under the MIT License - see LICENSE file for details

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

/// Create a test directory structure for integration testing
fn create_integration_test_structure() -> Result<TempDir, Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    // Create files with different sizes
    let mut large_file = File::create(base_path.join("large_file.txt"))?;
    large_file.write_all(&vec![b'L'; 5000])?; // 5KB

    let mut medium_file = File::create(base_path.join("medium_file.txt"))?;
    medium_file.write_all(&vec![b'M'; 2000])?; // 2KB

    let mut small_file = File::create(base_path.join("small_file.txt"))?;
    small_file.write_all(&vec![b'S'; 500])?; // 500 bytes

    // Create subdirectory with nested files
    fs::create_dir(base_path.join("subdir"))?;
    let mut nested_file = File::create(base_path.join("subdir/nested.txt"))?;
    nested_file.write_all(&vec![b'N'; 1000])?; // 1KB

    // Create empty directory
    fs::create_dir(base_path.join("empty_dir"))?;

    Ok(temp_dir)
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Disk Cleaner - Interactive Directory Analysis Tool"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_analyze_directory_basic() {
    let temp_dir = create_integration_test_structure().unwrap();
    
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg(temp_dir.path().to_str().unwrap())
        .arg("--depth")
        .arg("1");
    
    // Note: This will fail interactively, but we can test that it starts correctly
    cmd.timeout(std::time::Duration::from_secs(5));
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify it finds our test files
    assert!(stdout.contains("Directory Contents"));
    assert!(stdout.contains("large_file.txt") || stdout.contains("medium_file.txt"));
}

#[test]
fn test_depth_limiting() {
    let temp_dir = create_integration_test_structure().unwrap();
    
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg(temp_dir.path().to_str().unwrap())
        .arg("--depth")
        .arg("1");
    
    cmd.timeout(std::time::Duration::from_secs(5));
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // With depth 1, we should see the subdir but not necessarily its contents
    assert!(stdout.contains("subdir"));
}

#[test]
fn test_size_filtering() {
    let temp_dir = create_integration_test_structure().unwrap();
    
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg(temp_dir.path().to_str().unwrap())
        .arg("--min-size")
        .arg("1000") // 1KB minimum
        .arg("--depth")
        .arg("1");
    
    cmd.timeout(std::time::Duration::from_secs(5));
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show large and medium files, but not small file (500 bytes)
    assert!(stdout.contains("large_file.txt"));
    assert!(stdout.contains("medium_file.txt"));
}

#[test]
fn test_dirs_only_flag() {
    let temp_dir = create_integration_test_structure().unwrap();
    
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg(temp_dir.path().to_str().unwrap())
        .arg("--dirs-only")
        .arg("--depth")
        .arg("1");
    
    cmd.timeout(std::time::Duration::from_secs(5));
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should only show directories
    assert!(stdout.contains("subdir"));
    assert!(stdout.contains("empty_dir"));
}

#[test]
fn test_files_only_flag() {
    let temp_dir = create_integration_test_structure().unwrap();
    
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg(temp_dir.path().to_str().unwrap())
        .arg("--files-only")
        .arg("--depth")
        .arg("1");
    
    cmd.timeout(std::time::Duration::from_secs(5));
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should only show files, not directories
    assert!(stdout.contains("large_file.txt"));
    assert!(stdout.contains("medium_file.txt"));
    assert!(stdout.contains("small_file.txt"));
}

#[test]
fn test_mutually_exclusive_flags() {
    let temp_dir = create_integration_test_structure().unwrap();
    
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg(temp_dir.path().to_str().unwrap())
        .arg("--dirs-only")
        .arg("--files-only");
    
    // This should fail due to conflicting arguments
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_nonexistent_directory() {
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg("/definitely/does/not/exist");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_file_instead_of_directory() {
    let temp_dir = create_integration_test_structure().unwrap();
    let file_path = temp_dir.path().join("large_file.txt");
    
    let mut cmd = Command::cargo_bin("disk-cleaner-rs").unwrap();
    cmd.arg(file_path.to_str().unwrap());
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("is not a directory"));
}