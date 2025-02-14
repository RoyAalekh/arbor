cargo install dirtree
```

### From Source

```bash
# Clone the repository
git clone https://github.com/replit/dirtree
cd dirtree

# Build and install
cargo install --path .
```

### Platform-specific Notes

#### Windows
- Make sure you have the latest Rust toolchain installed via [rustup](https://rustup.rs/)
- Run in PowerShell or Command Prompt with administrator privileges for certain directories
- Windows Terminal is recommended for proper Unicode character display

#### macOS
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install dirtree
cargo install dirtree
```

#### Linux
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install required system dependencies (Ubuntu/Debian)
sudo apt-get install build-essential
# Install dirtree
cargo install dirtree
```

## Usage

### Basic Usage
```bash
# Show tree structure of current directory with sizes
dirtree

# Show tree structure of specific directory
dirtree -p /path/to/directory

# Export as PNG
dirtree -p /path/to/directory --png output.png

# Export as Mermaid flowchart
dirtree -p /path/to/directory --mermaid output.mmd
```

### Advanced Options
```bash
# Exclude specific patterns (affects size calculations)
dirtree -p /path/to/directory --exclude target --exclude node_modules

# Include only specific patterns
dirtree -p /path/to/directory --include "*.rs" --include "*.md"

# Limit directory depth
dirtree -p /path/to/directory --max-depth 3

# Follow symbolic links (includes link targets in size calculations)
dirtree -p /path/to/directory --follow-links
```

### Size Reporting

The tool shows sizes for both files and directories:
- Individual file sizes are displayed next to each file
- Directory sizes include the total size of all contained files (respecting exclude patterns)
- Sizes are shown in human-readable format (B, KB, MB, GB, TB, PB)
- Common build artifacts and system directories are automatically excluded from size calculations
- Use --exclude to exclude specific patterns from both display and size calculations

Example output:
```
. (1.5 MB)
├── src (150 KB)
│   ├── main.rs (2.5 KB)
│   └── lib.rs (1.2 KB)
└── docs (10 KB)
    └── README.md (1.5 KB)