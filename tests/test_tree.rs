use dirtree::tree::generate_tree;
use tempfile::TempDir;

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let exclude_patterns = vec![];
    let tree_data = generate_tree(temp_dir.path(), &exclude_patterns, &vec![], 0, false).unwrap();
    assert!(!tree_data.terminal_output.is_empty());
    assert!(tree_data.structure.is_empty());
}