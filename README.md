# ClocScan: counter of lines of code

ClocScan is a flexible and blazingly fast counter of lines of code.
Documentation is in the [docs/](docs/) folder, where you can find information
about the JSON configuration file options, and what should be put inside there.

**Disclaimer**: This program might not count lines of code accurately, so always
check the output and tweak your configuration file if needed.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [License](#license)
4.  [Contact](#contact)

## Usage

After installing, you can use this program to get the lines of code in your 
project. Here's how to use it, assuming you have already set-up the config


``` bash
./clocscan '/path/to/directory/' --config config.json
```

## Contributing

Contributiions are always welcome! If you feel like there is something you 
can improve, or if you'd like to add some features:

1.  Follow the [Rust Style
    Guide](https://doc.rust-lang.org/beta/style-guide/index.html)
2.  Follow the [code of conduct](CODE_OF_CONDUCT.md)
3.  Before committing changes, check the code for common mistakes with
    ``` console
    cargo clippy
    ```
4.  If you feel like there is some new features to add, feel free to open an
    issue.

## License

This project is dual-licensed under the [MIT](LICENSE_MIT.md) or [Apache
License](LICENSE_APACHE.md).

## Contact

If you have any questions, need further assistance, or would like to make
large changes to this repository, you can contact me at
`@winlogon.exe:matrix.org`.
