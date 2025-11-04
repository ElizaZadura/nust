# Nust Text Editor - Code Analysis

## Project Overview
This is a **Rust-based text editor** called "nust" (though the window title shows "Eliza Notes") built using the **egui** framework. It's described in the README as "One more 'text editor that's perfect for me' - project."

## Key Features
1. **Dual-pane interface**: Left and right text editing panes
2. **File operations**: Open, Save, and Save As functionality
3. **Keyboard shortcuts**:
   - `Ctrl+O`: Open file
   - `Ctrl+S`: Save current file
   - `Ctrl+Shift+S`: Save As
4. **File type support**: Text files, Markdown (.md), and log files (.log)
5. **Dirty state tracking**: Shows an asterisk (*) when files have unsaved changes

## Technical Stack
- **Language**: Rust (edition 2024)
- **GUI Framework**: egui 0.27 with eframe
- **Graphics Backend**: WGPU (WebGPU)
- **File Dialogs**: rfd (native file dialogs)
- **Error Handling**: anyhow

## Code Structure

### `Pane` struct
Represents each text editing pane with:
- File path and title
- Text content
- Dirty state tracking
- Methods for loading, saving, and saving-as operations

### `App` struct
Main application state with:
- Left and right panes
- Status message display
- File operation methods

### UI Layout
- **Top panel**: Menu buttons and status
- **Left side panel**: Left text editor (resizable, default 420px width)
- **Central panel**: Right text editor

## Current Limitations
1. **Focus tracking**: The code has a placeholder for focus tracking - currently defaults to saving the left pane
2. **Simple file handling**: Basic text file operations without advanced features
3. **No syntax highlighting**: Uses basic code editor styling
4. **No tabs**: Only two fixed panes

## Dependencies
- `eframe = { version = "0.27", features = ["default"] }`
- `egui = "0.27"`
- `rfd = "0.14"` (native file dialogs)
- `anyhow = "1"`

## Running the Application

### In WSL/Linux:
```bash
# Method 1: Use the provided script
./run_nust.sh

# Method 2: Manual with environment variables
source ~/.cargo/env
LIBGL_ALWAYS_SOFTWARE=1 MESA_GL_VERSION_OVERRIDE=3.3 cargo run

# Method 3: Development build
cargo run

# Method 4: Release build
cargo run --release
```

### WSL Display Issues:
If you encounter "Broken pipe" errors or display issues in WSL, use the environment variables:
- `LIBGL_ALWAYS_SOFTWARE=1` - Forces software rendering
- `MESA_GL_VERSION_OVERRIDE=3.3` - Sets OpenGL version

## File Structure
```
nust/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    └── main.rs
```

This appears to be a minimal, functional text editor focused on simplicity and ease of use, built as a personal project to create the "perfect" text editor for the developer's needs.