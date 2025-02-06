use crate::config::CodeFileConfig;
use anyhow::{Context, Result};
use clap::Parser;
use log::{info, LevelFilter};
use prettytable::{row, Table};
use rayon::prelude::*;
use regex::Regex;
use simple_logger::SimpleLogger;
use walkdir::WalkDir;

mod config;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

#[derive(Parser)]
struct Cli {
    #[arg(help = "The folder where the lines of code will be counted")]
    work_folder: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value = "config.json",
        help = "The JSON config file for code file extensions and ignore rules"
    )]
    config: PathBuf,

    #[arg(short, long, help = "Use info-level logging")]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    SimpleLogger::new()
        .with_level(if args.verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Warn
        })
        .init()
        .unwrap();

    info!("Logger initialized.");

    let binding = match args.work_folder {
        Some(p) => p.to_string_lossy().into_owned(),
        None => String::from("."),
    };
    let working_dir = &binding;
    let path = args.config.to_string_lossy().into_owned();
    let text = read_file_contents(&path)?;

    let code_file_config: CodeFileConfig = serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse JSON config file: {}", path))?;

    info!("Config file loaded: {}", path);

    let code_file_extensions: Vec<String> = code_file_config
        .code_file_extensions
        .iter()
        .map(|ext| ext.extension.clone())
        .collect();
    let code_file_regex =
        Regex::new(format!(".*\\.({})$", code_file_extensions.join("|")).as_str())
            .with_context(|| "ERROR: Failed to create regex!")?;

    let mut file_stats: HashMap<String, (String, usize, usize)> = HashMap::new();

    for entry in WalkDir::new(working_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let path = entry.path();
        let path_str = path.to_str().unwrap_or("");

        // Check if the path is a regular file and matches the code file extensions
        if path.is_file() && code_file_regex.is_match(path_str) {
            let reader = BufReader::new(File::open(path)?);

            let mut lines_count = 0;
            for line in reader.lines() {
                let line_str = line?;

                if is_comment_line(&line_str) {
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

        // Check if the path should be ignored based on folder and file rules
        if code_file_config
            .ignore
            .folders
            .par_iter()
            .any(|folder| path_str.contains(folder))
            || code_file_config
                .ignore
                .files
                .par_iter()
                .any(|file| path_str.matches(file).count() > 0)
        {
            continue;
        }
    }

    print_stats(&file_stats);
    Ok(())
}

/// Reads the contents of a file and returns them as a `String`.
pub fn read_file_contents(file_path: &str) -> Result<String> {
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Checks if a given line is a comment in various programming languages.
pub fn is_comment_line(line: &str) -> bool {
    let comment_patterns = ["//", "#", ";", "/*", "*/", "--"];

    let trimmed_line = line.trim();
    comment_patterns
        .iter()
        .any(|&pattern| trimmed_line.starts_with(pattern))
}

/// Prints the statistics in a pretty table format.
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
