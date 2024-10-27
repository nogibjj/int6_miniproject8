#[cfg(test)]
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempdir::TempDir;

#[test]
fn test_area() {
    let temp_dir = TempDir::new("temp_dir").expect("Failed to create temporary directory");
    let _temp_dir_path = temp_dir.path();

    // Test the area command with valid sides 3, 4, 5 (should = 6.00)
    let mut cmd = Command::cargo_bin("int6_miniproject7").unwrap();
    cmd.args(&["area", "--", "3", "4", "5"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Area of the triangle: 6.00"));
}

#[test]
fn test_perimeter() {
    let temp_dir = TempDir::new("temp_dir").expect("Failed to create temporary directory");
    let _temp_dir_path = temp_dir.path();

    // Test the perimeter command with valid sides 3, 4, 5 (should = 12.00)
    let mut cmd = Command::cargo_bin("int6_miniproject7").unwrap();
    cmd.args(&["perimeter", "--", "3", "4", "5"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Perimeter of the triangle: 12.00"));
}

#[test]
fn test_invalid_sides_area() {
    let temp_dir = TempDir::new("temp_dir").expect("Failed to create temporary directory");
    let _temp_dir_path = temp_dir.path();

    // Test the area command with invalid sides (e.g., 1, 1, 5), which can't form a valid triangle
    let mut cmd = Command::cargo_bin("int6_miniproject7").unwrap();
    cmd.args(&["area", "--", "1", "1", "5"]); // Corrected format
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid triangle sides provided."));
}

#[test]
fn test_invalid_sides_perimeter() {
    let temp_dir = TempDir::new("temp_dir").expect("Failed to create temporary directory");
    let _temp_dir_path = temp_dir.path();

    // Test the perimeter command with invalid sides (e.g., 1, 1, 5)
    let mut cmd = Command::cargo_bin("int6_miniproject7").unwrap();
    cmd.args(&["perimeter", "--", "1", "1", "5"]); // Corrected format
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid triangle sides provided."));
}
