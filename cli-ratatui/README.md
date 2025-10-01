# Directory Lister

A Rust CLI application built with ratatui that provides a terminal user interface for listing directory contents.

## Features

- **`--list`**: Interactive TUI mode showing current directory contents with file and folder icons
- **`--version`**: Display version information
- Clean, intuitive interface built with ratatui
- Cross-platform terminal support via crossterm

## Installation

Make sure you have Rust installed, then build the project:

```bash
cargo build --release
```

## Usage

### List Directory Contents (Interactive TUI)
```bash
./target/debug/directory-lister list
```

This opens an interactive terminal interface showing:
- 📁 Folders (with trailing `/`)
- 📄 Files
- Current directory path
- Navigation instructions (press 'q' or 'Esc' to quit)

### Show Version
```bash
./target/debug/directory-lister version
# or
./target/debug/directory-lister --version
```

### Help
```bash
./target/debug/directory-lister --help
```

## Dependencies

- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal manipulation
- **clap**: Command line argument parsing

## Controls (in TUI mode)

- `q` or `Esc`: Quit the application

## Build for Release

```bash
cargo build --release
```

The optimized binary will be available at `./target/release/directory-lister`.