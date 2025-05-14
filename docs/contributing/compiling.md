# Building from source

## Requirements

Before you begin, please ensure you have the following installed:

- **[Rust](https://rustup.rs/)**: Rust toolchain.
- **[Just](https://github.com/casey/just?tab=readme-ov-file#cross-platform)**: A command runner.
- **[Git](https://git-scm.com/downloads)**: For cloning the repository.

### Linux

[WebKitGTK](https://webkitgtk.org) is required for WRY to function on Linux. The package name may differ based on the operating system and Linux distribution:

```bash
# Arch Linux / Manjaro
sudo pacman -S webkit2gtk-4.1

# Debian / Ubuntu
sudo apt install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install gtk3-devel webkit2gtk4.1-devel
```

## Getting the source code

Clone the repository:

```bash
git clone https://github.com/doceazedo/godot_wry.git
cd godot_wry
```

## Building for your current platform

The simplest way to build the project is to run:

```bash
just build
```

This command will:

1. Detect your operating system and architecture.
2. Build the Rust library for your platform.
3. Copy the built files to the Godot project.

## Building for specific platforms

### macOS (Universal)

To build a universal binary (works on both Intel and Apple Silicon):

```bash
just build-macos-universal
```

### Other platforms

If you want to build for a specific platform other than your current one, you'll need to install the appropriate Rust target first:

```bash
# Linux
rustup target add x86_64-unknown-linux-gnu
just os="linux" build

# Windows
rustup target add x86_64-pc-windows-msvc
just os="windows" build

# macOS
rustup target add aarch64-apple-darwin
just os="macos" build
```

## Importing to a Godot project

If you are developing the extension itself, and want to test it or create examples/demos, the extension will already be available inside the Godot project located at the **"godot"** folder in the root of the repository.

To use the compiled extension in a different Godot project, just copy the entire **"godot/addons/godot_wry"** directory to your Godot project's **"addons"** folder:

```bash
mv godot/addons/godot_wry wherever/your/project/is/addons
```

You might need to restart the editor after importing the extension.

## Make sure it builds

When submitting a pull request, please ensure that your changes build successfully (at least) on your platform. You can check the GitHub workflow for reference on how we build the project in CI, if you want.
