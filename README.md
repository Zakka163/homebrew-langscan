# 🔍 LANGSCAN

A high-performance CLI tool to inventory languages, toolchains, and runtimes on your system. Built in Rust for maximum speed and reliability.

`langscan` helps developers quickly identify what programming environments are installed on their machine, providing versions and absolute paths for each detected language.

## 🚀 Features

- **Blazing Fast**: Native Rust implementation with minimal overhead.
- **Auto-Detection**: Scans system paths to find installed languages and tools.
- **Flexible Output**: Choose between pretty ASCII tables for humans or JSON for automation.
- **Cross-Platform**: Support for macOS (Intel & Apple Silicon) and Linux.

## 📦 Supported Languages

Langscan currently detects the following environments:

- **Rust** (`rustc`)
- **Go** (`go`)
- **Python** (`python3` / `python`)
- **Node.js** (`node`)
- **Java** (`java`)
- **Ruby** (`ruby`)
- **PHP** (`php`)

## 🛠️ Installation

### Homebrew (macOS & Linux)

The easiest way to install `langscan` is through Homebrew:

```bash
brew tap Zakka163/homebrew-langscan
brew install langscan
```

### 3. Manual Download (Linux)
If you don't want to use Homebrew, you can download the binary directly:

```bash
sudo curl -L \
https://github.com/Zakka163/homebrew-langscan/releases/latest/download/langscan-linux-amd64 \
-o /usr/local/bin/langscan
sudo chmod +x /usr/local/bin/langscan
```

### From Source

If you prefer to build it yourself, ensure you have [Rust](https://rustup.rs/) installed:

```bash
git clone https://github.com/Zakka163/homebrew-langscan.git
cd langscan
cargo build --release
```

The binary will be located at `target/release/langscan`.

## 💻 Usage

### Commands

| Command | Alias | Description |
| :--- | :--- | :--- |
| `scan` | - | Scan the system for installed languages |
| `--help` | `-h` | Show help information |
| `--version` | `-v` | Show version information |

### Examples

### 1. Scan for Languages

Inventory all programming languages on your system:

```bash
langscan scan
```

**Output:**
```text
┌──────────┬────────────┬──────────────────────────────────────────────────┐
│ Language │ Version    │ Path                                             │
├──────────┼────────────┼──────────────────────────────────────────────────┤
│ Rust     │ 1.82.0     │ /Users/user/.cargo/bin/rustc                     │
├──────────┼────────────┼──────────────────────────────────────────────────┤
│ Python   │ 3.12.3     │ /usr/local/bin/python3                           │
├──────────┼────────────┼──────────────────────────────────────────────────┤
│ Node.js  │ 22.5.1     │ /usr/local/bin/node                              │
└──────────┴────────────┴──────────────────────────────────────────────────┘
```

#### 2. JSON Output
Generate a machine-parsable JSON output, perfect for scripts or integration with other tools:

```bash
langscan scan --format json
```

**Output:**
```json
[
  {
    "name": "Rust",
    "version": "1.82.0",
    "toolchain": {
      "path": "/Users/user/.cargo/bin/rustc"
    }
  }
]
```

## ❓ Troubleshooting

### "failed to join paths" (macOS)
If you encounter errors related to `$DYLD_FALLBACK_LIBRARY_PATH` containing a colon, set a custom target directory:

```bash
CARGO_TARGET_DIR=/tmp/langscan_target cargo run -- scan
```

---

Made with ❤️ by [zakka163](https://github.com/Zakka163)
