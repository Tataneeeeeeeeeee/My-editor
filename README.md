# My-editor

A modern text editor developed in Rust with a graphical interface using GPUI (Zed's framework).

## Description

My-editor is a lightweight and performant text editor built with Rust. It offers a smooth editing experience with syntax highlighting, multi-tab support, an integrated file explorer, and a responsive user interface.

---

## Features

### Text Editing
- Character insertion and deletion
- `Tab` inserts 4 spaces
- `Enter` for new lines
- Full **UTF-8 / Unicode** support

### Syntax Highlighting
- Powered by **Syntect**
- Automatic detection by file extension
- Supported: Rust, C, C++, C#, Python, Markdown, TOML, JSON, TXT, and more

### Multi-tab Support
- Open multiple files simultaneously in separate tabs
- Tab bar with modified indicator (`●`)
- Switch, open, and close tabs with keyboard shortcuts

### File Explorer
- Integrated side panel showing the project directory tree
- Expand/collapse folders
- Click a file to open it in a new tab (or switch to it if already open)
- **Create a new file** directly from the explorer (inline input)
- **Create a new folder** directly from the explorer (inline input)
- File icons per language/type (Rust, C, C++, C#, Python, Markdown, TXT, lock…)
- Auto-refresh when the window regains focus
- Toggle the explorer panel with the sidebar button

### Keyboard Shortcuts
| Shortcut | Action |
|---|---|
| `←` `→` `↑` `↓` | Move cursor |
| `Home` / `End` | Go to start / end of line |
| `Backspace` | Delete previous character |
| `Enter` | New line |
| `Tab` | Insert 4 spaces |
| `Ctrl+N` | New tab |
| `Ctrl+T` | New tab |
| `Ctrl+W` | Close current tab |
| `Ctrl+Tab` | Switch to next tab |
| `Ctrl+O` | Open file |
| `Ctrl+S` | Save file |

### Mouse Support
- Click to position the cursor
- Scroll wheel to navigate vertically
- Scrollbar drag support
- Click on the explorer to open files or toggle folders

### File Management
- Open files via menu or `Ctrl+O` (dialog with filter by type)
- Save with `Ctrl+S` (uses existing path or opens a Save As dialog)
- Automatic file type detection for syntax highlighting

### Interface
- Dark theme (`#1e1e1e`)
- Menu bar (File: New, Open, Save)
- Tab bar with close button per tab
- Line numbers gutter
- Cursor position indicator (line / column)
- Auto-scroll to follow the cursor
- Activity bar with explorer toggle button

---

## Technologies

| Library | Role |
|---|---|
| **Rust** (2024 Edition) | Core language |
| **GPUI** | UI framework (from Zed) |
| **Syntect** | Syntax highlighting |
| **Cosmic Text** | Advanced text rendering |
| **RFD** | Native file dialogs |

---

## Installation

### Option 1 — From the release ZIP (recommended)

1. **Download** the latest `my-editor_amd64-linux.zip` from the releases page.

2. **Extract** the archive:
   ```bash
   unzip my-editor_amd64-linux.zip
   cd my-editor_amd64-linux
   ```

3. **Run the installer**:
   ```bash
   chmod +x install.sh
   ./install.sh
   ```
   The installer will:
   - Copy the binary to `/usr/local/share/my-editor/`
   - Create a symlink at `/usr/local/bin/my-editor`
   - Copy the assets to `/usr/local/share/my-editor/assets/`
   - Copy the settings to `~/.my-editor/`

4. **Launch the editor**:
   ```bash
   my-editor
   ```
   You can also open a specific folder directly:
   ```bash
   my-editor /path/to/your/project
   ```

---

### Option 2 — Build from source

#### Prerequisites
- Rust (stable toolchain)
- Cargo

#### Steps

1. Clone the repository:
   ```bash
   git clone <your-repo-url>
   cd My-editor
   ```

2. Build in release mode:
   ```bash
   cargo build --release
   ```

3. Run directly:
   ```bash
   cargo run --release
   # or with a folder:
   cargo run --release -- /path/to/your/project
   ```

---

## Project Structure

```
src/
├── main.rs                          # Application entry point
├── assets/
│   └── asset.rs                     # Asset loading
├── settings/
│   └── settings.rs                  # Configuration management
├── editor/
│   ├── mod.rs                       # Editor module
│   ├── editor_element.rs            # Editor UI component (rendering)
│   ├── editor_window.rs             # Main window + tab/file management
│   ├── syntax_highlighter.rs        # Syntax highlighting (Syntect)
│   ├── text_buffer.rs               # Text buffer & cursor management
│   ├── key/
│   │   ├── input.rs                 # Character & navigation input
│   │   ├── key.rs                   # Key event dispatcher
│   │   └── shortcuts.rs             # Ctrl+* keyboard shortcuts
│   ├── menu_bar/
│   │   ├── menu_bar.rs              # Menu bar state & actions
│   │   └── bar_element.rs           # Menu bar rendering
│   ├── tab_bar/
│   │   └── bar_element.rs           # Tab bar rendering
│   └── tool_bar/
│       ├── bar_element.rs           # Toolbar rendering
│       └── tree_file.rs             # File explorer (tree + icons)
└── window/
    ├── mod.rs                       # Window module
    └── window_render.rs             # AppState & window creation
```

---

## Architecture

- **TextBuffer** — text content, cursor position, scroll state, editing operations
- **SyntaxHighlighter** — syntax highlighting via Syntect, keyed by file extension
- **EditorElement** — UI component that renders the text area with line numbers
- **EditorWindow** — main window view: manages tabs, file explorer, key/mouse events, menu actions
- **FileTree** — directory tree structure with expand/collapse, file icons, and inline creation
- **MenuBar** — dropdown menu (File → New / Open / Save)
- **AppState** — global application state, window factory

---

## Development

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```
