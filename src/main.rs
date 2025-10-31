use crate::{
    config::CodeFileConfig,
    file_reading::{is_comment_or_empty, DEFAULT_CONFIG},
};
use anyhow::{Context, Result};
use async_walkdir::WalkDir;
use clap::{ArgAction, Parser};
use log::{debug, info, warn, LevelFilter};
use prettytable::{row, Table};
use regex::Regex;
use simple_logger::SimpleLogger;
use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
};

mod config;
mod file_reading;

use std::{collections::HashMap, path::PathBuf};

use futures::stream::StreamExt;

#[derive(Parser)]
struct Cli {
    /// The folder where the lines of code will be counted
    #[arg(default_value = ".")]
    work_folder: PathBuf,

    /// The JSON config file for code file extensions and ignore rules
    #[arg(short, long, default_value = "config.json")]
    config: PathBuf,

    /// Use logging (-v for warn, -vv for debug logging, or none to only print errors)
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    /// Show how much time it took to count the lines of code
    #[arg(short, long)]
    show_time_elapsed: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let filter = match args.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Debug,
        _ => panic!("Invalid option: use -v, -vv, or -vvv"),
    };
    SimpleLogger::new().with_level(filter).env().init().unwrap();

    let working_dir = args.work_folder.to_string_lossy().into_owned();
    let path = args.config.to_string_lossy().into_owned();
    let text = match std::fs::exists(&path) {
        Ok(true) => fs::read_to_string(&path).await?,
        _ => {
            warn!("Configuration file couldn't be found. Loading defaults");
            DEFAULT_CONFIG.to_string()
        }
    };

    let code_file_config: CodeFileConfig = serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse JSON config file: {}", path))?;

    info!("Config file loaded: {}", path);

    let code_file_extensions: Vec<String> = code_file_config
        .code_file_extensions
        .iter()
        .map(|ext| ext.extension.clone())
        .collect();

    let regex_string = format!(".*\\.({})$", code_file_extensions.join("|"));
    let code_file_regex = Regex::new(&regex_string)?;

    let mut file_stats: HashMap<String, (String, usize, usize)> = HashMap::new();

    info!("Going through files");
    let mut entries = WalkDir::new(working_dir);
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap_or("");

        debug!(
            "Checking if {} is a regular file and matches the code file extensions",
            path.display()
        );

        // Check if the path should be ignored based on folder and file rules
        let is_file_ignored = code_file_config
            .ignore
            .files
            .iter()
            .any(|file| path_str.contains(file));

        let is_folder_ignored = code_file_config
            .ignore
            .folders
            .iter()
            .any(|folder| path_str.contains(folder));

        if is_folder_ignored || is_file_ignored {
            continue;
        }
        if path.is_file() && code_file_regex.is_match(path_str) {
            let file = fs::File::open(&path).await?;
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            debug!("Counting lines for {}", path.display());
            let mut lines_count = 0;
            let mut in_multiline_comment = false;
            while let Some(line) = lines.next_line().await? {
                if is_comment_or_empty(
                    &line,
                    &code_file_config.comment_patterns,
                    &mut in_multiline_comment,
                ) {
                    continue;
                }
                lines_count += 1;
            }

            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
            let file_type = code_file_config
                .code_file_extensions
                .iter()
                .find(|ext| ext.extension == extension)
                .map(|ext| ext.file_type.clone())
                .unwrap_or_default();

            let entry = file_stats
                .entry(extension.to_string())
                .or_insert((file_type, 0, 0));
            entry.1 += 1;
            entry.2 += lines_count;
        }
    }

    print_stats(&file_stats);
    Ok(())
}

/// Print the statistics in a pretty table format.
fn print_stats(stats: &HashMap<String, (String, usize, usize)>) {
    let mut table = Table::new();
    table.add_row(row![
        "Extension",
        "File Type",
        "Number of Files",
        "Lines of Code"
    ]);

    for (ext, (file_type, count, lines)) in stats {
        table.add_row(row![ext, file_type, count, lines]);
    }

    table.printstd();
}
