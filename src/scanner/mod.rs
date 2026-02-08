use anyhow::Result;
use which::which;
use std::process::Command;
use crate::models::{Language, Toolchain, Component, ComponentKind};

pub trait Scanner {
    fn scan(&self) -> Result<Vec<Language>>;
}

pub struct PathScanner;

impl PathScanner {
    pub fn new() -> Self {
        Self
    }

    fn check_command(&self, cmd: &str, version_arg: &str) -> Option<String> {
        if let Ok(path) = which(cmd) {
            // Check version
            if let Ok(output) = Command::new(&path).arg(version_arg).output() {
                 if output.status.success() {
                     let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                     if !stdout.is_empty() {
                         return Some(stdout);
                     }
                     // Some tools (like Java) print version to stderr
                     let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                     if !stderr.is_empty() {
                         return Some(stderr);
                     }
                 }
            }
        }
        None
    }

    fn extract_version(&self, output: &str, tool: &str) -> String {
        let trimmed = output.trim();
        match tool {
            "rustc" => {
                // rustc 1.91.1 (ed61e7d7e 2025-11-07) -> 1.91.1
                trimmed.split_whitespace().nth(1).unwrap_or(trimmed).to_string()
            },
            "python" | "python3" => {
                // Python 3.13.5 -> 3.13.5
                trimmed.split_whitespace().nth(1).unwrap_or(trimmed).to_string()
            },
            "node" => {
                // v23.8.0 -> 23.8.0
                trimmed.trim_start_matches('v').to_string()
            },
            "java" => {
                // openjdk version "11.0.11" 2021-04-20 -> 11.0.11
                // OpenJDK Runtime Environment...
                // Only first line is passed here if using logic below, but let's handle the string
                 if let Some(first_line) = trimmed.lines().next() {
                    // split by " and take second element? or split by space
                    // java 11.0.11 2021-04-20 LTS
                    // openjdk version "11.0.11" ...
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
                // ruby 2.6.10p210 (2022-04-12 revision 67958) ... -> 2.6.10p210
                trimmed.split_whitespace().nth(1).unwrap_or(trimmed).to_string()
            },
            "php" => {
                // PHP 8.4.10 (cli) ... -> 8.4.10
                trimmed.split_whitespace().nth(1).unwrap_or(trimmed).to_string()
            },
             "go" => {
                // go version go1.24.0 darwin/arm64 -> 1.24.0
                if let Some(v_part) = trimmed.split_whitespace().nth(2) {
                    v_part.trim_start_matches("go").to_string()
                } else {
                    trimmed.to_string()
                }
            },
            _ => trimmed.to_string()
        }
    }

    fn create_language(&self, name: &str, cmd: &str, raw_version: String, kind: ComponentKind) -> Option<Language> {
        if let Ok(path) = which(cmd) {
            let version = self.extract_version(&raw_version, cmd);
            Some(Language {
                name: name.to_string(),
                version: version.clone(),
                toolchain: Toolchain {
                    path: path.clone(),
                    components: vec![
                        Component {
                            name: cmd.to_string(),
                            version,
                            path,
                            kind,
                        }
                    ]
                }
            })
        } else {
            None
        }
    }
}

impl Scanner for PathScanner {
    fn scan(&self) -> Result<Vec<Language>> {
        let mut languages = Vec::new();

        // Rust
        if let Some(version) = self.check_command("rustc", "--version") {
            if let Some(lang) = self.create_language("Rust", "rustc", version, ComponentKind::Compiler) {
                languages.push(lang);
            }
        }

        // Go
        if let Some(version) = self.check_command("go", "version") {
            if let Some(lang) = self.create_language("Go", "go", version, ComponentKind::Compiler) {
                languages.push(lang);
            }
        }

        // Python 3
        if let Some(version) = self.check_command("python3", "--version") {
             if let Some(lang) = self.create_language("Python", "python3", version, ComponentKind::Interpreter) {
                languages.push(lang);
            }
        } else if let Some(version) = self.check_command("python", "--version") {
             if let Some(lang) = self.create_language("Python", "python", version, ComponentKind::Interpreter) {
                languages.push(lang);
            }
        }

        // Node.js
        if let Some(version) = self.check_command("node", "--version") {
            if let Some(lang) = self.create_language("Node.js", "node", version, ComponentKind::Interpreter) {
                languages.push(lang);
            }
        }

        // Java
        if let Some(version) = self.check_command("java", "-version") {
            // Java output can be multiline, usually the first line is enough or we might want to capture more. 
            // For now, let's just take the first line if possible, or the whole thing.
            // The check_command trims it.
            // java -version output is often on stderr.
            // example: 
            // openjdk version "11.0.11" 2021-04-20
            // OpenJDK Runtime Environment (build 11.0.11+9-Ubuntu-0ubuntu2.20.04)
            // OpenJDK 64-Bit Server VM (build 11.0.11+9-Ubuntu-0ubuntu2.20.04, mixed mode, sharing)
            let lines: Vec<&str> = version.lines().collect();
            let first_line = lines.first().unwrap_or(&version.as_str()).to_string();
            
            if let Some(lang) = self.create_language("Java", "java", first_line, ComponentKind::Interpreter) { // Runtime/Interpreter
                languages.push(lang);
            }
        }

        // Ruby
        if let Some(version) = self.check_command("ruby", "--version") {
            if let Some(lang) = self.create_language("Ruby", "ruby", version, ComponentKind::Interpreter) {
                languages.push(lang);
            }
        }

        // PHP
        if let Some(version) = self.check_command("php", "--version") {
            // PHP 8.1.2 (cli) (built: Jan 24 2022 10:42:07) (NTS)
             let lines: Vec<&str> = version.lines().collect();
             let first_line = lines.first().unwrap_or(&version.as_str()).to_string();

            if let Some(lang) = self.create_language("PHP", "php", first_line, ComponentKind::Interpreter) {
                languages.push(lang);
            }
        }

        Ok(languages)
    }
}
