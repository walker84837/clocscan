# loc-counter-rs

This program counts the lines of code (LoC) in a directory.
Documentation is in the [docs/](docs/) folder, where you can find information about the JSON configuration file options, and what should be put inside there.

**Disclaimer**: This program might not count lines of code accurately, so always check the output and tweak your configuration file if needed.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [License](#license)
4.  [Acknowledgments](#acknowledgments)
5.  [Contact](#contact)

## Installation

To use this program, you'll need to install Rust and its package manager, Cargo. Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install) to get them set up.

Once Rust and Cargo are installed, you can build and install this program using the following command:

``` bash
cargo install --path .
```

or you can also do

``` bash
cargo build --release [opts]
```

## Usage

After installing, you can use this program to get the lines of code in your project. Here's how to use it, assuming you have already set-up config.json:

``` bash
./loc-counter-rs 'directory/' --config config.json
```

However, if you have set up some ignore settings, and you want to get rid of the "Ignoring: ..." text, you can run the program like:

``` bash
./loc-counter-rs 'directory/' --config config.json | grep -vi ignoring
```

## License

This project is dual-licensed under the [MIT](LICENSE_MIT.md) or [Apache License](LICENSE_APACHE.md).

## Acknowledgments

I'd like to give credit to the following libraries and tools used in this project:

  - [StructOpt](https://crates.io/crates/structopt) - for command-line argument parsing in Rust.
  - [WalkDir](https://crates.io/crates/walkdir) - for recursively walking into directories.
  - [Regex](https://crates.io/crates/regex) - for matching Regex patterns in Rust.
  - [Serde](https://crates.io/crates/serde) - for matching Regex patterns in Rust.

## Contact

If you have any questions, need further assistance, or would like to make large changes to this repository, you can contact me at <walker84837@gmail.com> or `@winlogon.exe:matrix.org` <!-- on Element Chat -->.

