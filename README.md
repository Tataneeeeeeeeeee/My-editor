# My-Editor

> A lightweight, performant text editor written in Rust with a modern GUI using GPUI (Zed's framework).

![Version](https://img.shields.io/badge/version-1.1.0-blue)
![Rust](https://img.shields.io/badge/rust-2024-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Recent Updates](#recent-updates)
- [Technologies](#technologies)
- [Installation](#installation)
- [Project Structure](#project-structure)
- [Architecture](#architecture)
- [Testing](#testing)
- [Development](#development)
- [Contributing](#contributing)
- [Requirements](#requirements)
- [Known Limitations](#known-limitations)
- [Acknowledgments](#acknowledgments)
- [Support](#support)

---

## Overview

My-Editor is a fast and responsive text editor designed for developers. Built entirely in Rust with GPUI, it combines a responsive UI with practical features like syntax highlighting, multi-tab support, an integrated file explorer, and customizable keyboard shortcuts.

---

## Features

### Core Editing
- **Character manipulation** — Insert and delete characters with full support for UTF-8 and Unicode
- **Smart indentation** — Press `Tab` to insert 4 spaces
- **Line breaks** — `Enter` creates new lines seamlessly
- **Full Unicode support** — Emoji, special characters, and international text

### Syntax Highlighting
- Powered by **Syntect** for accurate code highlighting
- Automatic language detection by file extension
- **Supported languages**: Rust, C, C++, C#, Python, JavaScript, Markdown, TOML, JSON, and plain text

### Multi-Tab Interface
- Open unlimited files in tabs simultaneously
- Visual modification indicator (`●`) shows unsaved changes
- Easy tab switching and closing with keyboard shortcuts
- Persistent tab state during a session

### File Explorer
- **Integrated side panel** displaying the project directory tree
- **Folder navigation** — Expand/collapse folders to browse your project
- **Quick file access** — Click to open files in new tabs or switch between them
- **File/folder creation** — Create new files and folders directly from the explorer
- **File icons** — Visual indicators for different file types (Rust, C, C++, C#, Python, JSON, lock files, etc.)
- **Auto-refresh** — Updates when the editor window regains focus
- **Toggle explorer** — Show/hide the sidebar with a single button

### Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `←` `→` `↑` `↓` | Navigate cursor |
| `Home` / `End` | Jump to line start / end |
| `Backspace` | Delete character before cursor |
| `Enter` | Insert new line |
| `Tab` | Insert 4 spaces |
| `Ctrl+N` | Create new tab |
| `Ctrl+T` | Create new tab |
| `Ctrl+W` | Close current tab |
| `Ctrl+Tab` | Switch to next tab |
| `Ctrl+O` | Open file dialog |
| `Ctrl+S` | Save current file |

### Mouse Support
- **Click positioning** — Click anywhere to move the cursor
- **Scrolling** — Scroll wheel for vertical navigation
- **Scrollbar interaction** — Drag the scrollbar or click to jump
- **File explorer** — Click to open files or toggle folders

### File Management
- **Open files** — Use `Ctrl+O` to browse and select files
- **Save files** — Press `Ctrl+S` to save (or Save As dialog for new files)
- **Auto-detection** — File type automatically detected for appropriate syntax highlighting
- **Recent files** — Quick access to recently opened files

### User Interface
- **Dark theme** — Eye-friendly dark color scheme (`#1e1e1e` background)
- **Menu bar** — File menu with New, Open, and Save options
- **Tab bar** — Visual representation of open files with close buttons
- **Line numbers** — Gutter showing line numbers for easy navigation
- **Status bar** — Displays current cursor position (line / column)
- **Smart scrolling** — Automatically scrolls to keep cursor visible
- **Activity bar** — Quick access buttons for explorer and other features

---

## Recent Updates

### Public Library Export
- Project now available as a reusable Rust library (`my_editor`)
- **Public modules**: `editor`, `settings`, `assets`, and `window` can be imported in other Rust projects
- Clean API for integration with other applications

### Comprehensive Test Suite
- **Unit tests** covering all major components:
  - `editor/` — Tests for text buffer, UI elements, window management, and syntax highlighting
  - `key/` — Input handling, keyboard events, and shortcuts (Ctrl+S, Ctrl+C, etc.)
  - `menu_bar/` — Menu bar functionality and rendering
  - `tab_bar/` — Tab management and navigation
  - `tool_bar/` — Toolbar buttons, search functionality, and file explorer

- **Integration tests**:
  - `settings/` — Configuration loading and management
  - `assets/` — Asset loading and resource management
  - `window/` — Window creation and rendering

- **Test execution**:
  ```bash
  cargo test                              # Run all tests
  cargo test -- --nocapture              # Show test output
  cargo test editor::text_buffer          # Run specific module tests
  cargo test editor::key::shortcuts       # Run specific test group
  ```

### Improved Modular Architecture
- Clear separation with `src/lib.rs` defining public interfaces
- Well-organized test structure in `tests/` directory
- Reusable components via library export
- Centralized test configuration in `tests/lib.rs`

---

## Technologies

| Technology | Purpose |
|---|---|
| **Rust** (2024 Edition) | Core programming language |
| **GPUI** | UI framework (from Zed text editor) |
| **Syntect** | Syntax highlighting engine |
| **Cosmic Text** | Advanced text rendering |
| **RFD** | Native file dialogs |
| **Serde & serde_json** | Configuration serialization |
| **Anyhow** | Error handling |

---

## Installation

### Option 1: Pre-built Binary (Recommended)

1. **Download** the latest release from the [releases page](../../releases):
   ```bash
   wget https://github.com/your-repo/My-editor/releases/download/v0.1.0/my-editor_amd64-linux.zip
   ```

2. **Extract** the archive:
   ```bash
   unzip my-editor_amd64-linux.zip
   cd my-editor_amd64-linux
   ```

3. **Run installer**:
   ```bash
   chmod +x install.sh
   ./install.sh
   ```
   
   This will:
   - Install the binary to `/usr/local/share/my-editor/`
   - Create a symlink at `/usr/local/bin/my-editor` for easy access
   - Copy assets to `/usr/local/share/my-editor/assets/`
   - Create settings directory at `~/.my-editor/`

4. **Launch the editor**:
   ```bash
   my-editor                      # Opens in current directory
   my-editor /path/to/project     # Opens specific folder
   ```

### Option 2: Build from Source

#### Requirements
- **Rust** toolchain (stable or latest)
- **Cargo** package manager
- **Linux/macOS/Windows** with standard development tools

#### Build Steps

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-repo/My-editor.git
   cd My-editor
   ```

2. **Build in release mode**:
   ```bash
   cargo build --release
   ```

3. **Run the editor**:
   ```bash
   cargo run --release
   # Or open a specific folder:
   cargo run --release -- /path/to/your/project
   ```

4. **Optional: Install locally**:
   ```bash
   cargo install --path .
   ```

---

## Project Structure

```
src/
├── main.rs                             # Application entry point
├── lib.rs                              # Public library interface
├── assets/
│   ├── mod.rs                          # Asset module
│   └── asset.rs                        # Asset loading & management
├── settings/
│   ├── mod.rs                          # Settings module
│   └── settings.rs                     # Configuration handling
├── editor/
│   ├── mod.rs                          # Editor module
│   ├── editor_element.rs               # Editor rendering component
│   ├── editor_window.rs                # Main window & tab management
│   ├── syntax_highlighter.rs           # Syntect-powered highlighting
│   ├── text_buffer.rs                  # Text storage & cursor control
│   ├── key/
│   │   ├── mod.rs
│   │   ├── input.rs                    # Character & navigation input
│   │   ├── key.rs                      # Key event routing
│   │   └── shortcuts.rs                # Keyboard shortcut handlers
│   ├── menu_bar/
│   │   ├── mod.rs
│   │   ├── menu_bar.rs                 # Menu state management
│   │   └── bar_element.rs              # Menu rendering
│   ├── tab_bar/
│   │   ├── mod.rs
│   │   └── bar_element.rs              # Tab bar rendering
│   └── tool_bar/
│       ├── mod.rs
│       ├── bar_element.rs              # Toolbar rendering
│       ├── tree_file.rs                # File explorer tree
│       ├── search_file.rs              # File search functionality
│       └── text_input.rs               # Text input component
└── window/
    ├── mod.rs                          # Window module
    └── window_render.rs                # AppState & window creation

tests/
├── lib.rs                              # Test configuration
├── assets/                             # Asset tests
├── editor/                             # Editor tests
├── settings/                           # Settings tests
└── window/                             # Window tests

assets/
├── rust_logo.png
├── python_logo.png
├── c_logo.png
├── cpp_logo.png
├── c-sharp_logo.png
├── markdown_logo.png
├── json_logo.png
├── txt_logo.png
├── lock_logo.png
├── directory_logo.png
├── explorer.png
├── search.png
├── new-document.png
└── new-folder.png
```

---

## Architecture

### Core Components

**TextBuffer**
- Manages text content and cursor position
- Handles scroll state and editing operations
- Supports undo/redo functionality
- Tracks line count and column position

**SyntaxHighlighter**
- Uses Syntect for accurate code highlighting
- Auto-detects language by file extension
- Caches syntax definitions for performance

**EditorElement**
- Renders the text area with line numbers
- Manages viewport and scrolling
- Handles cursor rendering and selection

**EditorWindow**
- Main application window container
- Manages tabs and file state
- Routes keyboard and mouse events
- Handles file I/O operations

**FileTree**
- Displays directory structure
- Supports folder expansion/collapse
- Provides file icons based on type
- Enables inline file/folder creation

**MenuBar**
- Provides File menu (New, Open, Save)
- Dispatches menu actions to editor

**AppState**
- Global application state management
- Window factory and lifecycle management

---

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run tests for specific module
cargo test editor::text_buffer
cargo test editor::key::
cargo test settings::

# Run a single test
cargo test test_text_buffer_new -- --exact

# Run tests with multiple threads
cargo test -- --test-threads=4

# Generate test coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

Test modules are organized to match source code structure:
- `tests/editor/` — Editor functionality
- `tests/settings/` — Configuration management
- `tests/assets/` — Asset loading
- `tests/window/` — Window rendering and state

---

## Development

### Setup
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and setup
git clone <repository-url>
cd My-editor
cargo build
```

### Building

```bash
# Development build (with debug symbols)
cargo build

# Optimized release build
cargo build --release

# Check for errors without building
cargo check
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint code
cargo clippy

# Generate documentation
cargo doc --open
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test editor::key::shortcuts
```

---

## Contributing

Contributions are welcome! Here's how to get started:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run `cargo fmt` and `cargo clippy`
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

---

## Requirements

- **Rust**: 1.70+ (2024 Edition)
- **OS**: Linux, macOS, or Windows
- **Memory**: 100MB+ recommended
- **Disk Space**: 500MB for build artifacts

---

## Known Limitations

- Undo/Redo not yet fully implemented
- Limited find/replace functionality
- No multi-cursor support
- Settings UI not yet implemented (JSON only)

---

## Acknowledgments

- [Zed](https://zed.dev/) for the GPUI framework
- [Syntect](https://github.com/trishume/syntect) for syntax highlighting
- The Rust community for excellent tools and libraries

---

## Support

For issues, bugs, or feature requests:
- Open an issue on [GitHub Issues](../../issues)
- Check existing issues for solutions
- Provide detailed reproduction steps for bugs
