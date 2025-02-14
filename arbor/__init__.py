import os
import sys
import subprocess

def main():
    """Find the Arbor binary and run it with CLI arguments."""
    bin_name = "arbor.exe" if sys.platform == "win32" else "arbor"
    bin_path = os.path.join(os.path.dirname(__file__), bin_name)

    if not os.path.exists(bin_path):
        print("Error: Arbor binary not found. Please reinstall.", file=sys.stderr)
        sys.exit(1)

    # Pass CLI arguments to the Rust binary
    subprocess.run([bin_path] + sys.argv[1:], check=False)

if __name__ == "__main__":
    main()
