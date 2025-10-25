# Pacshot Rust Application

## Overview
Pacshot is a simple Rust command-line application that allows users to create a snapshot of installed packages and reinstall them easily on an Arch Linux system using `pacman`.

## Features
- Create a snapshot of explicitly installed packages (`handle_ps` function).
- Reinstall packages from a snapshot file (`handle_us` function).
- Interactive mode with a simple text interface.
- ASCII banner for the application startup.

## Usage
1. **Build the application**

```bash
cargo build --release
```

2. **Run the application**

```bash
cargo run
```

3. **Modes:**
- **Create[1]**: Creates a snapshot of your installed packages and saves it to `pacs.pxs`.
- **Use[2]**: Reinstalls packages from a snapshot file. Prompts for the path.
- **Exit[3]**: Exit the application.

## Example
```text
What mode do you want? (Create[1] - Use[2] - Exit[3]) : 1
Snapshot saved successfully!
```

```text
What mode do you want? (Create[1] - Use[2] - Exit[3]) : 2
Enter path: pacs.pxs
0 - "package1" installed successfully! 9 to go ...
1 - "package2" installed successfully! 8 to go ...
...
```

## Dependencies
- Rust (1.80+ recommended)
- Arch Linux system with `pacman`

## File Structure
```
├── Cargo.toml
├── Cargo.lock
└── src
    └── main.rs
```

## Notes
- Uses `--noconfirm` and `--needed` flags for `pacman` to skip confirmation and avoid reinstalling already installed packages.
- Interactive input uses `stdin` with trimming and flushing.
- Lines read from snapshot file are processed individually to reinstall packages.
