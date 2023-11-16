# How the program works
    
Here's a quick explanation of how it all works, without having to read any code.

`loc-counter-rs` counts lines of code in specified files within a given folder, while allowing configuration via a JSON file for both file extensions to be considered and rules for ignoring certain folders and files.

This documentation file will be separated in a part which explains the [main.rs](../src/main.rs) file, and the [functions.rs](../src/functions.rs) file, with the program's usage.

## Table of Contents

  - [Usage](#usage)
  - [Files](#files)
      - [main.rs](#main)
      - [functions.rs](#functions)

## Usage

  - `work_folder`: The directory where the lines of code shall be counted will be searched recursively for any files in all directories and count each individual line.
  - `json_config`: The JSON configuration file outlines code file extensions and rules for ignoring.
  - `count_comments`: Option to count comments as lines of code or not.

## Files

### main

The [main.rs](../src/main.rs) file is responsible for parsing command-line arguments, reading a JSON configuration file (the is documentation [here](configuration.md)), and traversing the specified folder to count lines of code in files with designated extensions. Additionally, it provides options to include or exclude comments in the line count.

### functions

The [functions.rs](../src/functions.rs) module contains auxiliary functions used in the main code.

The functions inside it are the following:

  - `read_file_contents`: Reads the contents of a file, specified by its file path, and returns them as a string.

  - `is_comment_line`: This function determines whether a given line constitutes a comment in multiple programming languages.

This documentation is licensed under the GNU Free Documentation License (GFDL). These examples are licensed under the GNU General Public License version 3 (GPLv3).
