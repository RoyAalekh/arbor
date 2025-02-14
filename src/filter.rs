use std::path::Path;
use walkdir::DirEntry;

#[derive(Debug)]
pub struct ProjectType {
    pub name: String,
    excludes: Vec<String>,
}

impl ProjectType {
    fn new(name: &str, excludes: Vec<&str>) -> Self {
        ProjectType {
            name: name.to_string(),
            excludes: excludes.iter().map(|&s| s.to_string()).collect(),
        }
    }
}

pub fn detect_project_type(root: &Path) -> Option<ProjectType> {
    let project_types = vec![
        ("Node.js", vec!["package.json"], vec!["node_modules", "dist", ".next", "build"]),
        ("Python", vec!["requirements.txt", "pyproject.toml"], vec!["venv", "__pycache__", ".pytest_cache", "*.pyc"]),
        ("Rust", vec!["Cargo.toml"], vec!["target", "debug", "release"]),
        ("Java", vec!["pom.xml", "build.gradle"], vec!["target", "build", "*.class"]),
        ("Go", vec!["go.mod"], vec!["vendor", "bin"]),
    ];

    for (name, indicators, excludes) in project_types {
        if indicators.iter().any(|i| root.join(i).exists()) {
            return Some(ProjectType::new(name, excludes));
        }
    }

    None
}

pub fn should_include(entry: &DirEntry, exclude_patterns: &[String]) -> bool {
    let path = entry.path();

    // Skip hidden files and directories
    if is_hidden(path) {
        return false;
    }

    // Get project-specific excludes
    let project_excludes = if let Some(project_type) = detect_project_type(path.ancestors().find(|p| p.join("Cargo.toml").exists()).unwrap_or(Path::new("/"))) {
        project_type.excludes
    } else {
        vec![]
    };

    // Combine user excludes with project-specific excludes
    let all_excludes: Vec<String> = exclude_patterns.iter()
        .chain(project_excludes.iter())
        .cloned()
        .collect();

    // Skip common system directories and user-excluded patterns
    !all_excludes.iter().any(|pattern| {
        path.to_string_lossy()
            .to_string()
            .contains(pattern)
    })
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}