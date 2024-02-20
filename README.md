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

## Usage

After installing, you can use this program to get the lines of code in your 
project. Here's how to use it, assuming you have already set-up the config


``` bash
./loc-counter-rs 'directory/' --config config.json
```

## Contributing

Contributiions are always welcome! If you feel like there is something you 
can improve, or if you'd like to add some features:

1.  Follow the [Linux kernel coding style](https://docs.kernel.org/process/coding-style.html)
2.  Follow the [code of conduct](CODE_OF_CONDUCT.md)
3.  Keep code formatted with
    ``` console
    rustfmt --edition 2021
    ```
    and check the code for common mistakes with
    ``` console
    cargo clippy
    ```
4.  If you feel like there is some new features to add, feel free to open an
    issue.
5.  It is recommended that you use Rust stable rather than Rust
    nightly for a reliable development experience that is consistent with the
    broader ecosystem and suitable for production. You are advised to replace 
    all Rust nightly code in this repository with stable 
    counterparts.
6.  When using external libraries, it is recommended that you use lightweight 
    options such as `ureq` instead of `reqwest`.

## License

This project is dual-licensed under the [MIT](LICENSE_MIT.md) or [Apache License](LICENSE_APACHE.md).

## Acknowledgments

I'd like to give credit to the following libraries and tools used in this project:

  - [Clap](https://crates.io/crates/clap) - for command-line argument parsing 
    in Rust.
  - [WalkDir](https://crates.io/crates/walkdir) - for recursively walking 
    into directories.
  - [Regex](https://crates.io/crates/regex) - for matching Regex patterns in 
    Rust.
  - [Serde](https://crates.io/crates/serde) - for serializing and 
    deserializing Rust data structures efficiently and generically.

## Contact

If you have any questions, need further assistance, or would like to make
large changes to this repository, you can contact me at
<walker84837@gmail.com> or `@winlogon.exe:matrix.org`.
