# Departement Guessr

A simple guessing game for memorizing French departments.

## Rust Version

1. Install Rust from <https://rustup.rs>.
2. Build and run the GUI:

```bash
cargo run --release
```

Use the input fields to set a range of department codes and guess the names as they appear.

### GitHub Releases

Pushing a tag matching `v*.*.*` triggers a workflow that builds the Windows
binary and attaches it to a GitHub release. The compiled executable will be
available in the release assets.

## Python Prototype

The previous Python implementation using Tkinter is kept in `departement_guessr.py`.
