# LANGSCAN

A high-performance CLI tool to inventory languages, toolchains, and runtimes on your system. Built in Rust for maximum speed and reliability.

`langscan` helps developers quickly identify what programming environments are installed on their machine, providing versions and absolute paths for each detected language.

## Features

- **Blazing Fast**: Native Rust implementation with minimal overhead.
- **Auto-Detection**: Scans system paths to find installed languages and tools.
- **Flexible Output**: Choose between pretty ASCII tables for humans or JSON for automation.
- **Cross-Platform**: Support for macOS (Intel & Apple Silicon) and Linux.

## Supported Languages

Langscan currently detects 27+ programming environments and toolchains:

- **System**: Rust, Go, C (gcc), C++ (g++), Clang, Swift, Zig, Nim, Crystal, Dart
- **JVM**: Java, Kotlin, Scala
- **Scripting**: Python, Node.js, Bun, Deno, Ruby, PHP, Perl, Lua
- **Functional**: Elixir, Erlang, Clojure, Haskell (ghc)
- **Data Science**: Julia, R

## Installation

### Homebrew (macOS & Linux)

The easiest way to install `langscan` is through Homebrew:

```bash
brew tap Zakka163/homebrew-langscan
brew install langscan
```

### Manual Download

If you don't want to use Homebrew, you can download the binary directly for your platform:

```bash
# Linux
sudo curl -L https://github.com/Zakka163/homebrew-langscan/releases/latest/download/langscan-linux-amd64 -o /usr/local/bin/langscan

# macOS (Apple Silicon)
sudo curl -L https://github.com/Zakka163/homebrew-langscan/releases/latest/download/langscan-macos-arm64 -o /usr/local/bin/langscan

# macOS (Intel)
sudo curl -L https://github.com/Zakka163/homebrew-langscan/releases/latest/download/langscan-macos-amd64 -o /usr/local/bin/langscan

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

## Usage

### Commands

| Command | Description |
| :--- | :--- |
| `scan` | Scan the system for installed languages and tools |
| `version` | Print version information |
| `--help` | Show help information |

### Examples

#### 1. Basic Scan

Inventory all programming languages on your system in a formatted table:

```bash
langscan scan
```

**Output:**
```text
┌──────────┬────────────┬───────────┬──────────────────┬─────────────────────────────────────┐
│ Language │ Version    │ Size      │ Modified         │ Path                                │
├──────────┼────────────┼───────────┼──────────────────┼─────────────────────────────────────┤
│ Rust     │ 1.82.0     │ 12.5 MB   │ 2024-11-15 10:30 │ /Users/user/.cargo/bin/rustc         │
├──────────┼────────────┼───────────┼──────────────────┼─────────────────────────────────────┤
│ Node.js  │ 22.5.1     │ 111.6 MB  │ 2025-02-08 22:15 │ /usr/local/bin/node                 │
└──────────┴────────────┴───────────┴──────────────────┴─────────────────────────────────────┘
```

#### 2. Advanced Sorting

Sort the results by `name`, `size`, or `date` (modification time):

```bash
# Sort by size (largest first)
langscan scan --sort size

# Sort by modification date
langscan scan --sort date

# Multi-level sort (by name then size)
langscan scan --sort name,size
```

#### 3. Debugging Missing Languages

If a language isn't showing up, use the `--debug` flag to see where the tool is searching:

```bash
langscan scan --debug
```

#### 4. JSON Output

Generate machine-parsable JSON output for scripts or automation:

```bash
langscan scan --format json
```

## Troubleshooting

### Homebrew Link Conflict
If you installed `langscan` manually before using Homebrew, you might need to overwrite the link:

```bash
brew link --overwrite langscan
```

### "failed to join paths" (macOS)
If you encounter errors related to `$DYLD_FALLBACK_LIBRARY_PATH` containing a colon during development:

```bash
CARGO_TARGET_DIR=/tmp/langscan_target cargo run -- scan
```

---

Made with ❤️ by [zakka163](https://github.com/Zakka163)
