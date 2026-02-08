use clap::{CommandFactory, Parser, Subcommand};
use anyhow::Result;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

mod models;
mod scanner;

use crate::scanner::{Scanner, PathScanner};

#[derive(Parser)]
#[command(name = "langscan", version, about = "A tool to inventory languages and tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'v', long = "v")]
    version_flag: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan for languages and tools
    Scan {
        /// Output format (table, json)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version_flag {
        println!("langscan {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    match &cli.command {
        Some(Commands::Scan { format }) => {
            let scanner = PathScanner::new(); // Initialize scanner
            let languages = scanner.scan()?;

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&languages)?;
                    println!("{}", json);
                }
                _ => {
                    let mut table = Table::new();
                    table.load_preset(UTF8_FULL);
                    table.set_header(vec!["Language", "Version", "Path"]);

                    for lang in languages {
                        table.add_row(vec![
                            lang.name,
                            lang.version,
                            lang.toolchain.path.display().to_string(),
                        ]);
                    }

                    println!("{table}");
                }
            }
        }
        None => {
            Cli::command().print_help()?;
        }
    }

    Ok(())
}
