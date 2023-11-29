use anyhow::{Context, Result};
use regex::Regex;
use serde_derive::Deserialize;
use structopt::StructOpt;
use walkdir::WalkDir;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod functions;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The folder where the lines of code will be counted
    #[structopt(long = "folder", parse(from_os_str))]
    work_folder: PathBuf,

    /// The JSON config file for code file extensions and ignore rules
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    json_config: Option<PathBuf>,

    /// If this flag is present, with the value of true, it will ignore comments
    #[structopt(long = "ignore-comments")]
    ignore_comments: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct CodeFileConfig {
    code_file_extensions: Vec<String>,
    ignore: IgnoreConfig,
}

#[derive(Debug, Deserialize)]
struct IgnoreConfig {
    folders: Vec<String>,
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let working_dir: &str = args.work_folder.to_str().unwrap_or(".");

    let should_ignore_comments = match args.ignore_comments {
        Some(true) => true,
        Some(false) => false,
        None => false,
    };

    let path = args
        .json_config
        .as_ref()
        .and_then(|config| config.to_str())
        .unwrap_or("config.json");

    let text = functions::read_file_contents(path)?;

    let code_file_config: CodeFileConfig = serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse JSON config file: {}", path))?;

    let code_file_extensions = code_file_config.code_file_extensions;
    let code_file_regex =
        Regex::new(format!(".*\\.({})$", code_file_extensions.join("|")).as_str())
            .with_context(|| "Failed to create regex")?;

    let mut total_lines = 0;

    for entry in WalkDir::new(working_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let path = entry.path();
        let path_str = path.to_str().unwrap_or("");

        // Check if the path is a regular file and matches the code file extensions
        if path.is_file() && code_file_regex.is_match(path_str) {
            let file =
                File::open(path).with_context(|| format!("Failed to open file: {:#?}", path))?;
            let reader = BufReader::new(file);

            let mut lines_count = 0;

            for line in reader.lines() {
                let line_str = line?;

                // Skip lines that are comments
                if !should_ignore_comments && functions::is_comment_line(&line_str) {
                    continue;
                }

                lines_count += 1;
            }

            total_lines += lines_count;
            println!("File: {:#?}, Lines: {}", path, lines_count);
        }

        // Check if the path should be ignored based on folder and file rules
        if code_file_config
            .ignore
            .folders
            .iter()
            .any(|folder| path_str.contains(folder))
            || code_file_config
                .ignore
                .files
                .iter()
                .any(|file| path_str.matches(file).count() > 0)
        {
            println!("Ignoring: {:#?}", path);
        }
    }

    println!("Total lines of code: {}", total_lines);
    Ok(())
}
