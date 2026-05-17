use crate::{
    analyzer::{DEFAULT_CONFIG, is_comment_or_empty},
    config::CodeFileConfig,
};
use anyhow::{Context, Result};
use async_walkdir::WalkDir;
use clap::{ArgAction, Parser};
use log::{LevelFilter, debug, info, warn};
use prettytable::{Table, row};
use regex::Regex;
use simple_logger::SimpleLogger;
use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
};

mod analyzer;
mod config;

use std::{collections::HashMap, path::PathBuf};

use futures::stream::StreamExt;

#[derive(Debug)]
struct FileTypeStats {
    file_type: String,
    file_count: usize,
    total_sloc: usize,
}

impl FileTypeStats {
    fn new<S: Into<String>>(file_type: S) -> Self {
        Self {
            file_type: file_type.into(),
            file_count: 0,
            total_sloc: 0,
        }
    }

    fn accumulate_file(&mut self, sloc: usize) {
        self.file_count += 1;
        self.total_sloc += sloc;
    }
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(default_value = ".")]
    work_folder: PathBuf,

    #[arg(short, long, default_value = "config.json")]
    config: PathBuf,

    #[arg(short, action = ArgAction::Count)]
    verbose: u8,

    #[arg(short, long)]
    show_time_elapsed: bool,

    #[arg(short = 'e', long = "ext", value_delimiter = ',')]
    extensions: Vec<String>,

    #[arg(long = "sloc-only")]
    sloc_only: bool,
}

#[inline]
async fn load_config(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)
        .await
        .with_context(|| format!("Config file {path:?} not found or unreadable"))?;
    Ok(content)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let filter = setup_logging(args.verbose);
    SimpleLogger::new()
        .with_level(filter)
        .env()
        .init()
        .expect("failed to initialize logger");

    let path = args.config.to_string_lossy().into_owned();
    let text = match load_config(&path).await {
        Ok(t) => t,
        Err(e) => {
            warn!("{e:#}. Using default config.");
            DEFAULT_CONFIG.to_string()
        }
    };

    let code_file_config: CodeFileConfig = serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse JSON config file: {}", path))?;

    info!("Config file loaded: {}", path);

    let code_file_extensions = resolve_extensions(args.extensions, &code_file_config);

    if code_file_extensions.is_empty() {
        warn!("No extensions provided (via CLI or config). Nothing to count.");
        if args.sloc_only {
            println!("0");
        }
        return Ok(());
    }

    let regex_string = build_extension_regex(&code_file_extensions);
    let code_file_regex = Regex::new(&regex_string)?;

    let mut file_stats: HashMap<String, FileTypeStats> = HashMap::new();

    info!("Going through files");
    let mut entries = WalkDir::new(&args.work_folder);

    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();

        let Some(path_str) = path.to_str() else {
            warn!("Skipping non-UTF-8 path: {}", path.display());
            continue;
        };

        debug!(
            "Checking if {} is a regular file and matches the code file extensions",
            path.display()
        );

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
                let (is_comment, new_state) = is_comment_or_empty(
                    &line,
                    &code_file_config.comment_patterns,
                    in_multiline_comment,
                );
                in_multiline_comment = new_state;
                if is_comment {
                    continue;
                }
                lines_count += 1;
            }

            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default();

            let file_type = code_file_config
                .code_file_extensions
                .iter()
                .find(|ext| ext.extension == extension)
                .map(|ext| ext.file_type.clone())
                .unwrap_or_default();

            file_stats
                .entry(extension.to_string())
                .or_insert_with(|| FileTypeStats::new(file_type))
                .accumulate_file(lines_count);
        }
    }

    if args.sloc_only {
        let total_sloc: usize = file_stats.values().map(|stats| stats.total_sloc).sum();
        println!("{}", total_sloc);
        return Ok(());
    }

    print_stats(&file_stats);
    Ok(())
}

fn setup_logging(verbose: u8) -> LevelFilter {
    match verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Debug,
    }
}

fn resolve_extensions(cli_extensions: Vec<String>, config: &CodeFileConfig) -> Vec<String> {
    if !cli_extensions.is_empty() {
        cli_extensions
            .into_iter()
            .map(|mut ext| {
                if ext.starts_with('.') {
                    ext.remove(0);
                }
                ext
            })
            .collect()
    } else {
        config
            .code_file_extensions
            .iter()
            .map(|ext| ext.extension.clone())
            .collect()
    }
}

fn build_extension_regex(extensions: &[String]) -> String {
    let escaped: Vec<String> = extensions.iter().map(|e| regex::escape(e)).collect();
    format!(".*\\.({})$", escaped.join("|"))
}

fn print_stats(stats: &HashMap<String, FileTypeStats>) {
    let mut table = Table::new();
    table.add_row(row![
        "Extension",
        "File Type",
        "Number of Files",
        "Lines of Code"
    ]);

    let mut total_files = 0;
    let mut total_lines = 0;

    for (ext, file_stats) in stats {
        table.add_row(row![
            ext,
            file_stats.file_type,
            file_stats.file_count,
            file_stats.total_sloc
        ]);
        total_files += file_stats.file_count;
        total_lines += file_stats.total_sloc;
    }

    table.add_row(row!["---", "(total)", total_files, total_lines]);
    table.printstd();
}
