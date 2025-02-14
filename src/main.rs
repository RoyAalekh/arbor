//! # Show tree structure of current directory
//! dirtree
//! 
//! # Show tree structure of specific directory with exports
//! dirtree -p /path/to/directory --png output.png --mermaid output.mmd
//!

mod cli;
mod tree;
mod filter;
mod export;

use std::path::PathBuf;
use std::process;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    let path = args.path.unwrap_or_else(|| PathBuf::from("."));

    if !path.exists() {
        eprintln!("Error: Directory '{}' does not exist", path.display());
        process::exit(1);
    }

    let tree_data = match tree::generate_tree(
        &path,
        &args.exclude,
        &args.include,
        args.max_depth,
        args.follow_links,
    ) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error generating tree: {}", e);
            process::exit(1);
        }
    };

    // Display tree in terminal unless suppressed
    if !args.no_display {
        println!("{}", tree_data.terminal_output);
    }

    // Export as PNG if requested
    if let Some(png_path) = args.png {
        if let Err(e) = export::export_as_png(&tree_data.terminal_output, &png_path) {
            eprintln!("Error exporting PNG: {}", e);
            process::exit(1);
        }
        println!("Successfully exported PNG to: {}", png_path.display());
    }

    // Generate Mermaid if requested
    if let Some(mermaid_path) = args.mermaid {
        if let Err(e) = export::export_as_mermaid(&tree_data.structure, &mermaid_path) {
            eprintln!("Error generating Mermaid chart: {}", e);
            process::exit(1);
        }
        println!("Successfully exported Mermaid chart to: {}", mermaid_path.display());
    }
}