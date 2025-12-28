# Nust Text Editor

A simple, dual-pane text editor built with Rust and egui. Perfect for quick note-taking and text editing.

## Features

- **Dual-pane interface**: Left and right text editing areas with quick split/single toggles
- **Command palette first**: Press `Ctrl+Shift+P` to run any action (open, save, quick save, layout, close, etc.)
- **Keyboard-driven workflow**: Default shortcuts include `Ctrl+O/S/Shift+S/Alt+S`, `Ctrl+W`, and `Ctrl+1/2/3` for layout swaps
- **Quick saves**: Timestamped snapshots land in `target/quick_saves/` (or the system temp dir fallback)
- **Focus status**: Status bar shows which pane is active and reflects command results
- **WSL friendly**: Designed/tested on WSL2 (Ubuntu) with WSLg and the documented GL fallbacks

## Quick Start

### Prerequisites
- Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- WSL with GUI support (Windows 11 with WSLg recommended)

### Running the Application
```bash
# Easy way (uses the provided script)
./run_nust.sh

# Manual way
source ~/.cargo/env
LIBGL_ALWAYS_SOFTWARE=1 MESA_GL_VERSION_OVERRIDE=3.3 cargo run
```

## Command Palette & Shortcuts

- `Ctrl+Shift+P`: Open/close the palette. Type to filter, use `↑/↓`, press `Enter` to run.
- Layout shortcuts: `Ctrl+1` (left only), `Ctrl+2` (right only), `Ctrl+3` (split).
- File actions: `Ctrl+O` (open), `Ctrl+S` (save), `Ctrl+Shift+S` (save as), `Ctrl+Alt+S` (quick save), `Ctrl+W` (close focused pane).
- Palette entries mirror these actions so you can stay on the keyboard; status messages confirm every operation.

## WSL Troubleshooting Guide

### Common Issues and Solutions

#### 1. **"Broken pipe" or "Exit Failure: 1" errors**
**Problem**: GUI fails to display or crashes immediately
**Solutions**:
```bash
# Try software rendering
LIBGL_ALWAYS_SOFTWARE=1 cargo run

# Try different OpenGL version
MESA_GL_VERSION_OVERRIDE=3.3 cargo run

# Combine both
LIBGL_ALWAYS_SOFTWARE=1 MESA_GL_VERSION_OVERRIDE=3.3 cargo run
```

#### 2. **"linker cc not found" error**
**Problem**: Missing C compiler for Rust compilation
**Solution**:
```bash
sudo apt update && sudo apt install build-essential
```

#### 3. **"cargo not found" error**
**Problem**: Rust environment not sourced
**Solution**:
```bash
source ~/.cargo/env
# Or add to ~/.bashrc:
echo 'source ~/.cargo/env' >> ~/.bashrc
```

#### 4. **"starry sky" or visual artifacts around the window**
**Problem**: Graphics compositing issues in WSL
**Solutions**:
- This is cosmetic and doesn't affect functionality
- Try different environment variables (see #1)
- Update WSL: `wsl --update`

#### 5. **File dialogs don't work ("Save cancelled")**
**Problem**: Native file dialogs not supported in WSL
**Solution**: 
- Use the fallback input dialog that appears automatically
- Use "Quick Save" for timestamped files
- Use the manual save input field in the top menu

#### 6. **GUI doesn't appear at all**
**Problem**: No display server or WSLg not working
**Solutions**:
```bash
# Check if WSLg is working
echo $DISPLAY

# Try with X11 forwarding (if available)
export DISPLAY=:0
cargo run

# Update WSL
wsl --update
```

#### 7. **Performance issues or slow rendering**
**Problem**: Software rendering is slow
**Solutions**:
```bash
# Try hardware acceleration (if available)
MESA_GL_VERSION_OVERRIDE=4.5 cargo run

# Or stick with software rendering but reduce logging
RUST_LOG=warn cargo run
```

### System Requirements

#### Windows 11 (Recommended)
- WSL2 with WSLg support
- Windows 11 build 22000 or later
- GPU drivers up to date

#### Windows 10
- WSL2 with X11 server (VcXsrv, Xming, etc.)
- X11 forwarding configured
- May need additional setup for GUI applications

### Environment Variables Reference

| Variable | Purpose | Value |
|----------|---------|-------|
| `LIBGL_ALWAYS_SOFTWARE=1` | Force software rendering | Always use |
| `MESA_GL_VERSION_OVERRIDE=3.3` | Set OpenGL version | 3.3 or 4.5 |
| `RUST_LOG=warn` | Reduce logging verbosity | warn, error, or off |
| `DISPLAY=:0` | Set display server | :0 for WSLg |

### Quick Fixes Summary

1. **Always start with**: `LIBGL_ALWAYS_SOFTWARE=1 MESA_GL_VERSION_OVERRIDE=3.3 cargo run`
2. **If compilation fails**: Install build tools with `sudo apt install build-essential`
3. **If GUI doesn't show**: Check WSLg with `echo $DISPLAY`
4. **If file dialogs fail**: Use the built-in input dialogs or Quick Save
5. **If performance is poor**: Try different OpenGL versions or stick with software rendering

### Getting Help

If you encounter issues not covered here:
1. Check the status bar for error messages
2. Try running with `RUST_LOG=debug` for more information
3. Ensure WSL is updated: `wsl --update`
4. Check Windows GPU drivers are up to date
5. 

*Is this a bad idea? It is. Of course it is. It's exactly how cursed architectures are born. As long as current state is - it works, and I like it - that's good enough for me. You have been warned.*

The application is designed to be robust and provide fallbacks for common WSL limitations.
