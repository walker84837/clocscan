# JSON Configuration

This document explains how to configure the JSON file used for specifying code
file extensions and files/folders to ignore.

The JSON file defines two main sections: **Code File Extensions** and **What to
Ignore**.

## Table of Contents

   - [Code file extensions](#code-file-extensions)
   - [What to ignore](#what-to-ignore)
       - [Folders](#folders)
       - [Files](#files)

### Code File Extensions

This part details the file extensions that the program needs to identify as code
files. It will only calculate the number of lines of code in the files that have
these specific extensions. Additionally, each extension is associated with a
file type.

Here's an example:

```json
"code_file_extensions": [
    {"extension": "rs", "file_type": "Rust"},
    {"extension": "py", "file_type": "Python"},
    {"extension": "js", "file_type": "JavaScript"}
]
```

In this example, the program will count lines of code in Rust, Python, and
JavaScript files only.

### What to Ignore

This section explains how to exclude certain folders and files from code line
counting. This helps avoid parts of your project that you don't want to include
in the count, like `README.md` files or `.gitignore`.

#### Folders

Specify the names of the folders to be ignored. If any part of the folder path
contains one of these names, the program will ignore the file.

Here's an example:

```json
"ignore": {
    "folders": ["test", "docs"]
}
```

#### Files

Specify the names of specific files to be ignored. The program will check if the
filename matches one of these patterns and ignore it accordingly.

**Example:**

```json
"ignore": {
    "files": ["README.md", "ignore_this.py"]
}
```

*This documentation is licensed under the GNU Free Documentation License (GFDL). These examples are licensed under the MIT License.*
