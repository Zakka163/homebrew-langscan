# LANGSCAN

A high-performance CLI tool to inventory languages, toolchains, and runtimes on your system.
Built in Rust for speed and reliability.

## 🚀 Features

- **Blazing Fast**: Written in Rust for maximum performance and low memory footprint.
- **Accurate Detection**: Identifies installed languages, versions, and paths.
- **Flexible Output**: Supports both human-readable ASCII tables and machine-parsable JSON.
- **Cross-Platform**: Designed for macOS and Linux (Windows support coming).

## 📦 Supported Languages

LangScan currently detects:

- **Rust** (`rustc`)
- **Go** (`go`)
- **Python** (`python3` / `python`)
- **Node.js** (`node`)
- **Java** (`java`)
- **Ruby** (`ruby`)
- **PHP** (`php`)

## 🛠️ Installation

### From Source

Ensure you have Rust installed (via [rustup](https://rustup.rs/)).

```bash
git clone <repository-url>
cd langscan
cargo build --release
```

The binary will be available at `target/release/langscan`.

## 💻 Usage

### Basic Scan (Table Output)

Run a quick scan to see a neat table of installed languages:

```bash
cargo run -- scan
# Or if built:
./target/release/langscan scan
```

**Output Example:**

```text
┌──────────┬────────────┬──────────────────────────────────────────────────┐
│ Language ┆ Version    ┆ Path                                             │
╞══════════╪════════════╪══════════════════════════════════════════════════╡
│ Rust     ┆ 1.91.1     ┆ /Users/mymac/.cargo/bin/rustc                    │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Python   ┆ 3.13.5     ┆ /usr/local/bin/python3                           │
├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Node.js  ┆ 23.8.0     ┆ /Users/mymac/.nvm/versions/node/v23.8.0/bin/node │
└──────────┴────────────┴──────────────────────────────────────────────────┘
```

### JSON Output (For Integrations)

Generate a JSON output for use in scripts, CI/CD pipelines, or other tools.

```bash
cargo run -- scan --format json
```

**Output Example:**

```json
[
  {
    "name": "Rust",
    "version": "1.91.1",
    "toolchain": {
      "path": "/Users/mymac/.cargo/bin/rustc",
      "components": [
        {
          "name": "rustc",
          "version": "1.91.1",
          "path": "/Users/mymac/.cargo/bin/rustc",
          "kind": "Compiler"
        }
      ]
    }
  }
]
```

## ❓ Troubleshooting

### "failed to join paths" Error (macOS)

If you see an error like:
```text
error: failed to join paths from `$DYLD_FALLBACK_LIBRARY_PATH` together ... Caused by: path segment contains separator `:`
```

This occurs if your project path contains a colon (`:`).

**Solution:**
Set `CARGO_TARGET_DIR` to a safe location:

```bash
CARGO_TARGET_DIR=/tmp/langscan_target cargo run -- scan
```

---

Made with ❤️ by zakka163
