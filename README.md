# Pacshot - Snapshot Manager

## Overview
Pacshot is a simple Rust command-line application that allows users to create a snapshot of installed packages and reinstall them easily on an Linux system using `pacman` or `apt`.

## Features
- Create a snapshot of explicitly installed packages.
- Reinstall packages from a snapshot file.
- Interactive mode with a simple text interface.

## Usage

Install prebuilt versions or build from source :

1. **Build the application**

```bash
cargo build --release
```

2. **Run the application**

```bash
cargo run
```

3. **Modes:**
- **Create[1]**: Creates a snapshot of your installed packages and saves it to `[name].pxs`.
- **Use[2]**: Reinstalls packages from a snapshot file. Prompts for the path.
- **Exit[3]**: Exit the application.

## Dependencies
- Rust (1.80+ recommended)
- Linux system with `pacman` or `apt`
