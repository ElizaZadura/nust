#!/bin/bash
# Script to run Nust text editor with proper WSL settings

# Source Rust environment
source ~/.cargo/env

# Set environment variables for better WSL compatibility
export LIBGL_ALWAYS_SOFTWARE=1
export MESA_GL_VERSION_OVERRIDE=3.3

# Run the application
cargo run