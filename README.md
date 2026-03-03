# My-editor

A modern text editor developed in Rust with a graphical interface using GPUI (Zed's framework).

**Version: 1.0.0**

## 📋 Description

My-editor is a lightweight and performant text editor built with Rust. It offers a smooth editing experience with syntax highlighting, intuitive navigation, and a responsive user interface.

## ✨ Features

- **Complete text editing**: Insertion, deletion, cursor navigation
- **Syntax highlighting**: Support for multiple languages via Syntect
- **Keyboard navigation**: 
  - Arrow keys to move the cursor
  - Home/End keys support
  - Line-by-line navigation
- **File management**:
  - Open existing files
  - Save documents
  - Automatic file type detection for syntax highlighting
- **Modern interface**: 
  - Intuitive menu bar
  - Line numbers
  - Cursor position indicator
  - Auto-scroll to follow the cursor
- **UTF-8 support**: Full Unicode character handling

## 🛠️ Technologies

- **Rust** (2024 Edition)
- **GPUI**: Modern UI framework from Zed
- **Syntect**: Syntax highlighting
- **Cosmic Text**: Advanced text rendering
- **RFD**: Native file dialogs

## 📦 Installation

### Prerequisites

- Rust (stable version recommended)
- Cargo

### Installation Steps

1. Clone the repository:
```bash
git clone <your-repo-url>
cd My-editor
```

2. Build the project:
```bash
cargo build --release
```

3. Run the editor:
```bash
cargo run --release
```

## 🚀 Usage

### Launch

```bash
cargo run
```

### Keyboard Shortcuts

- **Editing**:
  - `Backspace`: Delete previous character
  - `Tab`: Insert tab (4 spaces)
  - `Enter`: New line

- **Navigation**:
  - `←` `→` `↑` `↓`: Move cursor
  - Mouse click: Position cursor

- **Files**:
  - File menu → "Open": Open a file
  - File menu → "Save": Save the document

## 📁 Project Structure

```
src/
├── main.rs                          # Application entry point
├── editor/
│   ├── mod.rs                       # Editor module
│   ├── editor_element.rs            # Editor component
│   ├── editor_window.rs             # Editor window
│   ├── menu_bar.rs                  # Menu bar
│   ├── syntax_highlighter.rs        # Syntax highlighting
│   ├── text_buffer.rs               # Text buffer management
│   └── text_layout.rs               # Text layout
└── window/
    ├── mod.rs                       # Window module
    └── window_render.rs             # Window rendering
```

## 🧩 Architecture

The project is organized into distinct modules:

- **TextBuffer**: Manages text content, cursor position, and editing operations
- **SyntaxHighlighter**: Provides syntax highlighting using Syntect
- **EditorElement**: UI component responsible for rendering the editor
- **EditorWindow**: Manages the main window and events
- **MenuBar**: Menu bar with actions (Open, Save)
- **AppState**: Global application state

## 🔧 Development

### Debug build

```bash
cargo build
```

### Release build

```bash
cargo build --release
```

### Run tests

```bash
cargo test
```
