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

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use futures::stream::StreamExt;

struct FileTypeStats {
    /// A human-readable label for this file type, e.g. "Rust Source"
    file_type: String,
    /// Number of files found with this extension
    file_count: usize,
    /// Total SLOC (source lines of code) across all those files
    total_sloc: usize,
}

impl FileTypeStats {
    pub fn new<S: Into<String>>(file_type: S) -> Self {
        Self {
            file_type: file_type.into(),
            file_count: 0,
            total_sloc: 0,
        }
    }

    /// Add a single file’s line count to this type’s totals.
    pub fn accumulate_file(&mut self, sloc: usize) {
        self.file_count += 1;
        self.total_sloc += sloc;
    }
}

#[derive(Parser)]
struct Cli {
    /// The folder where the lines of code will be counted
    #[arg(default_value = ".")]
    work_folder: PathBuf,

    /// The JSON config file for code file extensions and ignore rules
    #[arg(short, long, default_value = "config.json")]
    config: PathBuf,

    /// Use logging (-v for warn, -vv for debug logging, or none to only print errors)
    #[arg(short, action = ArgAction::Count)]
    verbose: u8,

    /// Show how much time it took to count the lines of code
    #[arg(short, long)]
    show_time_elapsed: bool,

    /// Only look for the given file extension(s). Can be repeated: `-e rs -e py` or `-e rs,py`.
    /// Provide extensions without a leading dot (rs, py) or with a dot (.rs).
    #[arg(short = 'e', long = "ext", value_delimiter = ',')]
    extensions: Vec<String>,

    /// Print only the total SLOC number (non-empty, non-comment lines) and exit.
    #[arg(long = "sloc-only")]
    sloc_only: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let filter = match args.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => panic!("Invalid option: use -v, -vv, or -vvv"),
    };
    SimpleLogger::new().with_level(filter).env().init().unwrap();

    let working_dir = args.work_folder.to_string_lossy().into_owned();
    let path = args.config.to_string_lossy().into_owned();

    let text = load_config(&path).await;

    let code_file_config: CodeFileConfig = serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse JSON config file: {}", path))?;

    info!("Config file loaded: {}", path);

    // Determine which extensions to use: CLI-provided ones override config
    let code_file_extensions: Vec<String> = if !args.extensions.is_empty() {
        args.extensions
            .into_iter()
            .map(|mut ext| {
                // normalize: remove leading dot if present
                if ext.starts_with('.') {
                    ext.remove(0);
                }
                ext
            })
            .collect()
    } else {
        code_file_config
            .code_file_extensions
            .iter()
            .map(|ext| ext.extension.clone())
            .collect()
    };

    if code_file_extensions.is_empty() {
        // nothing to look for
        warn!("No extensions provided (via CLI or config). Nothing to count.");
        if args.sloc_only {
            println!("0");
        }
        // Just exit without printing any table, we have no config
        return Ok(());
    }

    // Look for anything ending in any of the configured extensions
    let regex_string = format!(".*\\.({})$", code_file_extensions.join("|"));
    let code_file_regex = Regex::new(&regex_string)?;

    let mut file_stats: HashMap<String, FileTypeStats> = HashMap::new();

    info!("Going through files");
    let mut entries = WalkDir::new(working_dir);

    // Start walking into the specified folder to look for lines to count
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap_or_default();

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

        // Ignore the file or folder if it's ignored
        if is_folder_ignored || is_file_ignored {
            continue;
        }

        // If the current entry is a file and matches the regex specified above...
        if path.is_file() && code_file_regex.is_match(path_str) {
            let file = fs::File::open(&path).await?;
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            debug!("Counting lines for {}", path.display());
            let mut lines_count = 0;
            // Count the lines of the file, keeping in mind single-line comments and multi-line
            // comments
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

            // Get the file extension from the path
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default();

            // Look for the file type alias from the extension in the config
            let file_type = code_file_config
                .code_file_extensions
                .iter()
                .find(|ext| ext.extension == extension)
                .map(|ext| ext.file_type.clone())
                .unwrap_or_default();

            // Add the file to the stats by mutating the entry
            file_stats
                .entry(extension.to_string())
                .or_insert_with(|| FileTypeStats::new(file_type))
                .accumulate_file(lines_count);
        }
    }

    // If --sloc-only: print a single number (total SLOC)
    if args.sloc_only {
        let total_sloc: usize = file_stats.values().map(|stats| stats.total_sloc).sum();
        println!("{}", total_sloc);
        return Ok(());
    }

    print_stats(&file_stats);
    Ok(())
}

/// Print the statistics in a pretty table format.
fn print_stats(stats: &HashMap<String, FileTypeStats>) {
    let mut table = Table::new();
    table.add_row(row![
        "Extension",
        "File Type",
        "Number of Files",
        "Lines of Code"
    ]);

    let mut total_files = 0usize;
    let mut total_lines = 0usize;

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

async fn load_config<P: AsRef<Path> + std::fmt::Debug>(path: P) -> String {
    let file_exists = std::fs::exists(&path)
        .map_err(|e| {
            warn!("Couldn't verify config existence at {path:?}: {e}. Using defaults.");
            e
        })
        .unwrap_or(false);

    let config = if file_exists {
        match fs::read_to_string(path.as_ref()).await {
            Ok(content) => content,
            Err(e) => {
                warn!("Failed to read config file {path:?}: {e}. Using defaults.");
                DEFAULT_CONFIG.to_string()
            }
        }
    } else {
        warn!("Config file not found at {path:?}. Creating new default config.");
        DEFAULT_CONFIG.to_string()
    };

    config
}
