from setuptools import setup
import sys
import os

# Detect the platform and select the correct binary name
bin_name = "arbor.exe" if sys.platform == "win32" else "arbor"

# Define the binary path inside the package
bin_path = os.path.join("arbor", bin_name)

setup(
        name="arbor-cli",
        version="0.1.0",
        description="A fast, cross-platform directory tree viewer",
        author="Your Name",
        author_email="your.email@example.com",
        license="MIT",
        packages=["arbor"],
        package_data={"arbor": [bin_name]},  # Include the Rust binary
        install_requires=[],
        entry_points={
            "console_scripts": [
                "arbor=arbor:main",  # This makes `arbor` callable from CLI
            ],
        },
)
