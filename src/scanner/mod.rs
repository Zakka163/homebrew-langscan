use anyhow::Result;
use which::which;
use std::process::Command;
use std::time::UNIX_EPOCH;
use crate::models::{Language, Toolchain, Component, ComponentKind};

pub trait Scanner {
    fn scan(&self) -> Result<Vec<Language>>;
}

pub struct PathScanner {
    debug: bool,
}

impl PathScanner {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    fn debug_log(&self, msg: &str) {
        if self.debug {
            eprintln!("[DEBUG] {}", msg);
        }
    }

    fn find_path(&self, cmd: &str) -> Option<std::path::PathBuf> {
        // First try standard which
        if let Ok(path) = which(cmd) {
            self.debug_log(&format!("Found {} in PATH: {}", cmd, path.display()));
            return Some(path);
        }

        // Try common paths
        let common_paths = vec![
            "/opt/homebrew/bin",
            "/usr/local/bin",
            "/usr/bin",
            "/bin",
            "/usr/sbin",
            "/sbin",
        ];

        for prefix in common_paths {
            let path = std::path::Path::new(prefix).join(cmd);
            if path.exists() {
                self.debug_log(&format!("Found {} in common path: {}", cmd, path.display()));
                return Some(path);
            }
        }

        // Try common version managers for node
        if cmd == "node" {
            if let Ok(home) = std::env::var("HOME") {
                let home_path = std::path::Path::new(&home);
                
                // NVM
                let nvm_dir = home_path.join(".nvm/versions/node");
                if nvm_dir.exists() {
                    if let Ok(entries) = std::fs::read_dir(nvm_dir) {
                        for entry in entries.flatten() {
                            let bin_path = entry.path().join("bin").join("node");
                            if bin_path.exists() {
                                self.debug_log(&format!("Found node in NVM path: {}", bin_path.display()));
                                return Some(bin_path);
                            }
                        }
                    }
                }

                // FNM
                let fnm_dir = home_path.join(".local/share/fnm/node-versions");
                if fnm_dir.exists() {
                    if let Ok(entries) = std::fs::read_dir(fnm_dir) {
                        for entry in entries.flatten() {
                            let bin_path = entry.path().join("installation/bin/node");
                            if bin_path.exists() {
                                self.debug_log(&format!("Found node in FNM path: {}", bin_path.display()));
                                return Some(bin_path);
                            }
                        }
                    }
                }

                // ASDF
                let asdf_dir = home_path.join(".asdf/installs/nodejs");
                if asdf_dir.exists() {
                    if let Ok(entries) = std::fs::read_dir(asdf_dir) {
                        for entry in entries.flatten() {
                            let bin_path = entry.path().join("bin/node");
                            if bin_path.exists() {
                                self.debug_log(&format!("Found node in ASDF path: {}", bin_path.display()));
                                return Some(bin_path);
                            }
                        }
                    }
                }
            }
        }

        self.debug_log(&format!("Could not find command: {}", cmd));
        None
    }

    fn check_command(&self, cmd: &str, version_arg: &str) -> Option<(std::path::PathBuf, String)> {
        if let Some(path) = self.find_path(cmd) {
            // Check version
            match Command::new(&path).arg(version_arg).output() {
                Ok(output) => {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if !stdout.is_empty() {
                            return Some((path, stdout));
                        }
                        // Some tools (like Java) print version to stderr
                        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                        if !stderr.is_empty() {
                            return Some((path, stderr));
                        }
                    } else {
                        self.debug_log(&format!("Command {} failed with status: {}", cmd, output.status));
                    }
                }
                Err(e) => {
                    self.debug_log(&format!("Failed to execute {}: {}", cmd, e));
                }
            }
        }
        None
    }

    fn extract_version(&self, output: &str, tool: &str) -> String {
        let trimmed = output.trim();
        match tool {
            "rustc" => {
                trimmed.split_whitespace().nth(1).unwrap_or(trimmed).to_string()
            },
            "python" | "python3" => {
                trimmed.split_whitespace().nth(1).unwrap_or(trimmed).to_string()
            },
            "node" => {
                trimmed.trim_start_matches('v').to_string()
            },
            "java" => {
                 if let Some(first_line) = trimmed.lines().next() {
                    if first_line.contains('"') {
                         first_line.split('"').nth(1).unwrap_or(first_line).to_string()
                    } else {
                         first_line.split_whitespace().nth(2).unwrap_or(first_line).to_string()
                    }
                 } else {
                     trimmed.to_string()
                 }
            },
            "ruby" => {
                trimmed.split_whitespace().nth(1).unwrap_or(trimmed).to_string()
            },
            "php" => {
                trimmed.split_whitespace().nth(1).unwrap_or(trimmed).to_string()
            },
             "go" => {
                if let Some(v_part) = trimmed.split_whitespace().nth(2) {
                    v_part.trim_start_matches("go").to_string()
                } else {
                    trimmed.to_string()
                }
            },
            _ => trimmed.to_string()
        }
    }

    fn create_language(&self, name: &str, cmd: &str, path: std::path::PathBuf, raw_version: String, kind: ComponentKind) -> Language {
        let version = self.extract_version(&raw_version, cmd);
        
        // Fetch metadata
        let metadata = std::fs::metadata(&path).ok();
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified_at = metadata.as_ref()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
        let created_at = metadata.as_ref()
            .and_then(|m| m.created().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        Language {
            name: name.to_string(),
            version: version.clone(),
            size,
            modified_at,
            toolchain: Toolchain {
                path: path.clone(),
                components: vec![
                    Component {
                        name: cmd.to_string(),
                        version,
                        path,
                        kind,
                        size,
                        created_at,
                        modified_at,
                    }
                ]
            }
        }
    }
}

impl Scanner for PathScanner {
    fn scan(&self) -> Result<Vec<Language>> {
        if self.debug {
            if let Ok(path) = std::env::var("PATH") {
                self.debug_log(&format!("Current PATH: {}", path));
            }
        }
        let mut languages = Vec::new();

        // Rust
        if let Some((path, version)) = self.check_command("rustc", "--version") {
            languages.push(self.create_language("Rust", "rustc", path, version, ComponentKind::Compiler));
        }

        // Go
        if let Some((path, version)) = self.check_command("go", "version") {
            languages.push(self.create_language("Go", "go", path, version, ComponentKind::Compiler));
        }

        // Python 3
        if let Some((path, version)) = self.check_command("python3", "--version") {
             languages.push(self.create_language("Python", "python3", path, version, ComponentKind::Interpreter));
        } else if let Some((path, version)) = self.check_command("python", "--version") {
             languages.push(self.create_language("Python", "python", path, version, ComponentKind::Interpreter));
        }

        // Node.js
        if let Some((path, version)) = self.check_command("node", "--version") {
            languages.push(self.create_language("Node.js", "node", path, version, ComponentKind::Interpreter));
        }

        // Java
        if let Some((path, version)) = self.check_command("java", "-version") {
            let first_line = version.lines().next().unwrap_or(&version).to_string();
            languages.push(self.create_language("Java", "java", path, first_line, ComponentKind::Interpreter));
        }

        // Ruby
        if let Some((path, version)) = self.check_command("ruby", "--version") {
            languages.push(self.create_language("Ruby", "ruby", path, version, ComponentKind::Interpreter));
        }

        // PHP
        if let Some((path, version)) = self.check_command("php", "--version") {
             let first_line = version.lines().next().unwrap_or(&version).to_string();
            languages.push(self.create_language("PHP", "php", path, first_line, ComponentKind::Interpreter));
        }

        Ok(languages)
    }
}
