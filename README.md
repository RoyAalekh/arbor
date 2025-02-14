## 🌳 **Arbor - A Fast & Cross-Platform Directory Tree Viewer**

**Arbor** is a **lightweight, blazing-fast CLI tool** for visualizing directory structures in a **tree format** with file sizes. Built with Rust, it is optimized for **speed and efficiency** across **Windows, macOS, and Linux**.

---

## 🔥 **Features**
👉 **Cross-platform**: Works on Windows, macOS, and Linux.  
👉 **Tree View with Sizes**: Displays directory hierarchies with human-readable file sizes.  
👉 **Export Options**: Generate PNG and Mermaid diagrams for better visualization.  
👉 **Custom Filtering**: Exclude or include files and directories using patterns.  
👉 **Follow Symbolic Links**: Optionally follow symlinks for a complete structure.  
👉 **Fast & Lightweight**: Built with Rust for high performance.

---

## 🚀 **Installation**

### **📚 Install via pip (Recommended)**
Install **Arbor** directly from PyPI:
```sh
pip install arbor-cli
```
or install **directly from GitHub**:
```sh
pip install git+https://github.com/RoyAalekh/arbor.git
```

### **🧜🏻 Install via Cargo**
If you have Rust installed, you can also install Arbor via Cargo:
```sh
cargo install arbor
```

---

## 📂 **Usage**
Arbor is a **command-line tool**, run it like:
```sh
arbor -p /path/to/directory
```

### **🔹 Basic Commands**
```sh
# Show tree structure of current directory
arbor

# Show tree structure of a specific directory
arbor -p /path/to/directory

# Export as PNG
arbor -p /path/to/directory --png output.png

# Export as Mermaid flowchart
arbor -p /path/to/directory --mermaid output.mmd
```

### **🔹 Advanced Options**
```sh
# Exclude specific patterns (e.g., ignore 'node_modules' or 'target')
arbor -p /path/to/directory --exclude node_modules --exclude target

# Include only specific file types (e.g., show only Rust and Markdown files)
arbor -p /path/to/directory --include "*.rs" --include "*.md"

# Limit directory depth (e.g., show only 3 levels deep)
arbor -p /path/to/directory --max-depth 3

# Follow symbolic links
arbor -p /path/to/directory --follow-links
```

---

## 📊 **Example Output**
Example tree view of a sample project directory:

```
. (1.5 MB)
├── src (150 KB)
│   ├── main.rs (2.5 KB)
│   └── lib.rs (1.2 KB)
└── docs (10 KB)
    └── README.md (1.5 KB)
```

---

## 🛠 **Building from Source**
If you want to build Arbor manually from source, clone the repository:
```sh
git clone https://github.com/RoyAalekh/arbor.git
cd arbor
cargo build --release
```
Move the binary to `/usr/local/bin` (Linux/macOS) or `C:\Windows\System32` (Windows) for global access:
```sh
sudo mv target/release/arbor /usr/local/bin/arbor  # Linux/macOS
move target\release\arbor.exe C:\Windows\System32\arbor.exe  # Windows
```

---

## 🔄 **Uninstallation**
To remove Arbor from your system:
```sh
pip uninstall arbor-cli  # If installed via pip
cargo uninstall arbor  # If installed via Cargo
```

---

## 💡 **Why Arbor?**
🚀 **Faster** than `tree`, with file sizes included.  
📚 **Cross-platform** – Works on all major OS.  
📊 **Perfect for documentation** – Exportable as PNG & Mermaid.  
🔍 **Filter, exclude, and customize your tree view**.

---

## 💖 **Contributing**
We welcome contributions! Feel free to submit **issues** and **pull requests** on GitHub.

### **🌐 Repository**: [GitHub - Arbor](https://github.com/RoyAalekh/arbor)

1. **Fork the repository**
2. **Create a new branch** (`git checkout -b feature-branch`)
3. **Commit your changes** (`git commit -m "Add new feature"`)
4. **Push to GitHub** (`git push origin feature-branch`)
5. **Create a Pull Request**

---

## 🐝 **License**
**MIT License** – Feel free to use, modify, and distribute!

---

## 🎉 **Get Started Now!**
Run:
```sh
arbor -p .
```
And visualize your directory structures like a pro! 🚀

---

