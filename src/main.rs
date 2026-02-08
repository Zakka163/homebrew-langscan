use clap::{CommandFactory, Parser, Subcommand};
use anyhow::Result;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use chrono::{DateTime, Local};

mod models;
mod scanner;

use crate::scanner::{Scanner, PathScanner};
use crate::models::Language;

#[derive(Parser)]
#[command(name = "langscan", version, about = "A tool to inventory languages and tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan for languages and tools
    Scan {
        /// Output format (table, json)
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Show debug information
        #[arg(short, long)]
        debug: bool,

        /// Sort results (name, size, date, modified). Can be comma-separated.
        #[arg(short, long)]
        sort: Option<String>,
    },
    /// Print version information
    Version,
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn format_date(secs: Option<u64>) -> String {
    secs.and_then(|s| {
        DateTime::from_timestamp(s as i64, 0).map(|dt| {
            let local_dt: DateTime<Local> = DateTime::from(dt);
            local_dt.format("%Y-%m-%d %H:%M").to_string()
        })
    }).unwrap_or_else(|| "N/A".to_string())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Scan { format, debug, sort }) => {
            let scanner = PathScanner::new(*debug);
            let mut languages = scanner.scan()?;

            // Sorting logic
            if let Some(sort_keys) = sort {
                let keys: Vec<&str> = sort_keys.split(',').collect();
                languages.sort_by(|a, b| {
                    for key in &keys {
                        let cmp = match key.trim() {
                            "name" => a.name.cmp(&b.name),
                            "size" => b.size.cmp(&a.size), // Sort size descending by default
                            "modified" | "date" => b.modified_at.cmp(&a.modified_at), // Sort date descending by default
                            _ => std::cmp::Ordering::Equal,
                        };
                        if cmp != std::cmp::Ordering::Equal {
                            return cmp;
                        }
                    }
                    std::cmp::Ordering::Equal
                });
            }

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&languages)?;
                    println!("{}", json);
                }
                _ => {
                    let mut table = Table::new();
                    table.load_preset(UTF8_FULL);
                    table.set_header(vec!["Language", "Version", "Size", "Modified", "Path"]);

                    for lang in languages {
                        table.add_row(vec![
                            lang.name,
                            lang.version,
                            format_size(lang.size),
                            format_date(lang.modified_at),
                            lang.toolchain.path.display().to_string(),
                        ]);
                    }

                    println!("{table}");
                }
            }
        }
        Some(Commands::Version) => {
            println!("langscan {}", env!("CARGO_PKG_VERSION"));
        }
        None => {
            Cli::command().print_help()?;
        }
    }

    Ok(())
}
