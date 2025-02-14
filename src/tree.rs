use std::path::Path;
use walkdir::{WalkDir, DirEntry};
use ansi_term::Colour::{Blue, White, Green, Yellow};
use crate::filter;
use std::fs;

pub struct TreeData {
    pub terminal_output: String,
    pub structure: Vec<(String, String)>, // (parent, child) relationships for Mermaid
}

pub struct TreeStats {
    total_size: u64,
    total_files: usize,
    total_dirs: usize,
}

fn format_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

fn calculate_dir_size(path: &Path, exclude_patterns: &[String]) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| filter::should_include(e, exclude_patterns))
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| fs::metadata(entry.path()).ok())
        .map(|metadata| metadata.len())
        .sum()
}

fn calculate_tree_stats(entries: &[DirEntry]) -> TreeStats {
    let mut stats = TreeStats {
        total_size: 0,
        total_files: 0,
        total_dirs: 0,
    };

    for entry in entries {
        let path = entry.path();
        if entry.file_type().is_file() {
            stats.total_files += 1;
            if let Ok(metadata) = fs::metadata(path) {
                stats.total_size += metadata.len();
            }
        } else if entry.file_type().is_dir() {
            stats.total_dirs += 1;
        }
    }

    stats
}

pub fn generate_tree(
    root: &Path,
    exclude_patterns: &[String],
    include_patterns: &[String],
    max_depth: usize,
    follow_links: bool,
) -> Result<TreeData, Box<dyn std::error::Error>> {
    let mut output = String::new();
    let mut relationships = Vec::new();

    // Add project type detection header
    if let Some(project_type) = filter::detect_project_type(root) {
        output.push_str(&format!("Detected project type: {} (auto-excluding common build artifacts)\n\n", 
            Blue.bold().paint(project_type.name)));
    }

    let root_size = calculate_dir_size(root, exclude_patterns);
    output.push_str(&format!("{} ({})\n",
        Blue.bold().paint("."),
        Green.paint(format_size(root_size))));

    let mut walker = WalkDir::new(root).follow_links(follow_links);
    if max_depth > 0 {
        walker = walker.max_depth(max_depth);
    }

    let entries: Vec<DirEntry> = walker
        .into_iter()
        .filter_entry(|e| filter::should_include(e, exclude_patterns))
        .filter_map(|e| e.ok())
        .filter(|e| {
            if include_patterns.is_empty() {
                true
            } else {
                let path = e.path().to_string_lossy();
                include_patterns.iter().any(|pattern| {
                    path.contains(pattern) || e.file_type().is_dir()
                })
            }
        })
        .collect();

    for entry in &entries {
        let depth = entry.depth();
        let path = entry.path();

        if depth == 0 {
            continue;
        }

        let prefix = if depth > 1 {
            "│   ".repeat(depth - 1)
        } else {
            String::new()
        };

        let is_last = is_last_in_directory(entry, &entries);
        let connector = if is_last { "└── " } else { "├── " };

        let display_name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let size = if path.is_dir() {
            calculate_dir_size(path, exclude_patterns)
        } else {
            fs::metadata(path).map(|m| m.len()).unwrap_or(0)
        };

        let size_str = format_size(size);
        let formatted_line = if entry.file_type().is_dir() {
            format!("{}{}{} ({})\n",
                prefix,
                connector,
                Blue.paint(display_name.to_string()),
                Green.paint(size_str))
        } else {
            format!("{}{}{} ({})\n",
                prefix,
                connector,
                White.paint(display_name.to_string()),
                Green.paint(size_str))
        };

        output.push_str(&formatted_line);

        // Store relationship for Mermaid chart
        if let Some(parent) = path.parent() {
            if let (Some(parent_name), Some(child_name)) = (
                parent.file_name().map(|n| n.to_string_lossy().to_string()),
                path.file_name().map(|n| n.to_string_lossy().to_string())
            ) {
                relationships.push((parent_name, child_name));
            }
        }
    }

    // Add summary section
    let stats = calculate_tree_stats(&entries);
    output.push_str(&format!("\n{}\n", Yellow.paint("Summary:")));
    output.push_str(&format!("  Total size: {}\n", Green.paint(format_size(stats.total_size))));
    output.push_str(&format!("  Files: {}\n", stats.total_files));
    output.push_str(&format!("  Directories: {}\n", stats.total_dirs));

    Ok(TreeData {
        terminal_output: output,
        structure: relationships,
    })
}

fn is_last_in_directory(entry: &DirEntry, all_entries: &[DirEntry]) -> bool {
    let parent = entry.path().parent().unwrap();
    let siblings: Vec<_> = all_entries.iter()
        .filter(|e| e.path().parent() == Some(parent))
        .collect();

    siblings.last().map(|last| last.path() == entry.path()).unwrap_or(true)
}