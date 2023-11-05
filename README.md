# loc-counter-rs

This program counts the lines of code (LoC) in a directory.

**Disclaimer**: This program might not be production-ready, so don't expect this program to work as you might expect.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)
5.  [Acknowledgments](#acknowledgments)
6.  [Contact](#contact)

## Installation

To use this program, you'll need to install Rust and its package manager, Cargo. Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install) to get them set up.

Once Rust and Cargo are installed, you can build and install this program using the following command:

``` bash
cargo install --path .
```

## Usage

After installing, you can use this program to normalize text files. Here's how to use it:

``` bash
./loc-counter-rs 'directory/'
```

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix: `git switch -c newbranch`.
3.  Make your changes and commit them.
4.  Push your changes to your fork: `git push origin newbranch`.
5.  Create a pull request to the `main` branch of the original repository.

## License

This project is dual-licensed under the [MIT](LICENSE_MIT.md) or [Apache License](LICENSE_APACHE.md).

## Acknowledgments

I'd like to give credit to the following libraries and tools used in this project:

  - [StructOpt](https://crates.io/crates/structopt) - for command-line argument parsing in Rust.
  - [WalkDir](https://crates.io/crates/walkdir) - for recursively walking into directories.
  - [Regex](https://crates.io/crates/regex) - for matching Regex patterns in Rust.

## Contact

If you have any questions, need further assistance, or would like to make large changes to this repository, you can contact me at <walker84837@gmail.com> or `@winlogon.exe:matrix.org` <!-- on Element Chat -->.

