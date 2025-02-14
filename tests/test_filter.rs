use dirtree::filter::{detect_project_type, should_include};
use std::fs::{self, File};
use tempfile::TempDir;

#[test]
fn test_project_type_detection() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Test Rust project detection
    File::create(root.join("Cargo.toml")).unwrap();
    let project = detect_project_type(root).unwrap();
    assert_eq!(project.name, "Rust");

    // Cleanup
    fs::remove_file(root.join("Cargo.toml")).unwrap();

    // Test Node.js project detection
    File::create(root.join("package.json")).unwrap();
    let project = detect_project_type(root).unwrap();
    assert_eq!(project.name, "Node.js");

    // Test no project detection
    fs::remove_file(root.join("package.json")).unwrap();
    assert!(detect_project_type(root).is_none());
}

#[test]
fn test_file_filtering() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create test directory structure
    fs::create_dir(root.join("src")).unwrap();
    fs::create_dir(root.join("target")).unwrap();
    fs::create_dir(root.join(".git")).unwrap();
    File::create(root.join("Cargo.toml")).unwrap();
    File::create(root.join("src/main.rs")).unwrap();

    let exclude_patterns = vec!["test".to_string()];

    // Test that source files are included
    let entry = walkdir::WalkDir::new(root.join("src"))
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    assert!(should_include(&entry, &exclude_patterns));

    // Test that target directory is excluded (Rust project auto-exclude)
    let entry = walkdir::WalkDir::new(root.join("target"))
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    assert!(!should_include(&entry, &exclude_patterns));

    // Test that hidden directories are excluded
    let entry = walkdir::WalkDir::new(root.join(".git"))
        .into_iter()
        .next()
        .unwrap()
        .unwrap();
    assert!(!should_include(&entry, &exclude_patterns));
}