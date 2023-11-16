# JSON Configuration

This document explains how to configure the JSON file used for specifying exceptions, and which file extensions to ignore.

## Table of Contents

  - [Code file configuration](#file-configuration)
      - [File extensions](#extensions)
      - [What to ignore](#what-to-ignore)
          - [Folders](#folders)
          - [Files](#files)

## File configuration

The JSON file defines two main sections: **Extensions** and **What to ignore**.

### Extensions

This part details the certain file extensions that the program needs to identify as code files. It will only calculate the number of lines of code in the files that have these specific extensions.
Here's an example:

``` json
"code_file_extensions": ["rs", "py", "js"]
```

For this example, the program will count lines of code in Rust, Python, and JavaScript files only.

### What to ignore

This section explains how to exclude certain folders and files from code line counting. This helps avoid parts of your project that you don't want to include in the count, like `README.md` files, or `.gitignore`.

#### Folders

Specify the names of the folders to be ignored. If any part of the folder path contains one of these names, the program will ignore the file.
Here's an example:

``` json
"ignore": {
    "folders": ["test", "docs"]
}
```

In this example, the program will ignore any files found within folders containing the names "test" or "docs."

#### Files

Specify the names of specific files to be ignored. The program will check if the filename matches one of these patterns and ignore it accordingly.

**Example:**

``` json
"ignore": {
    "files": ["README.md", "ignore_this.py"]
}
```

In this example, the program will ignore files named "README.md" or "ignore\_this.py".

This documentation is licensed under the GNU Free Documentation License (GFDL). These examples are licensed under the GNU General Public License, version 3.
