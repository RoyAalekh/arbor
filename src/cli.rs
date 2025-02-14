//! Command-line interface definition for the Arbor application.
//! 
//! This module defines the command-line arguments and options available
//! to users when running the application.

use clap::Parser;
use std::path::PathBuf;

/// A command-line tool for generating and exporting directory tree structures
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Optional path to directory (defaults to current directory)
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Maximum depth of the tree (0 for unlimited)
    #[arg(short, long, default_value = "0")]
    pub max_depth: usize,

    /// Patterns to exclude (e.g., "node_modules", "target")
    /// Can be specified multiple times for different patterns
    #[arg(short, long)]
    pub exclude: Vec<String>,

    /// Patterns to include (e.g., "*.rs", "*.md")
    /// Can be specified multiple times for different patterns
    #[arg(short, long)]
    pub include: Vec<String>,

    /// Export tree as PNG image
    /// Specify the output file path (e.g., "tree.png")
    #[arg(long)]
    pub png: Option<PathBuf>,

    /// Export as Mermaid flowchart
    /// Specify the output file path (e.g., "tree.mmd")
    #[arg(long)]
    pub mermaid: Option<PathBuf>,

    /// Suppress terminal output
    /// Useful when only generating exports
    #[arg(long)]
    pub no_display: bool,

    /// Follow symbolic links
    #[arg(long)]
    pub follow_links: bool,
}