# ClocScan: counter of lines of code

[![Rust](https://github.com/walker84837/clocscan/actions/workflows/rust.yml/badge.svg)](https://github.com/walker84837/clocscan/actions/workflows/rust.yml)

ClocScan is a flexible and blazingly fast counter of lines of code. Documentation is in the [docs/](docs/) folder, where you can find information about the JSON configuration file options, and what should be put inside there.

> [!WARNING]
> This project is still in development.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [License](#license)
4.  [Contact](#contact)

## Usage

After installing, you can use this program to get the lines of code in your project. Here's some example usage:

``` console
./clocscan '/path/to/directory/'
```

## Contributing

Contributiions are always welcome! If you feel like there is something you can improve, or if you'd like to add some features, feel free to make a PR or open an issue!

The documentation of the code is at <https://walker84837.github.io/clocscan/>.

### Roadmap

- [ ] Add more file types in the example config file
- [ ] Make the output table look prettier
- [ ] Add an option to just output the total lines of code and more options (for scripting purposes)

## License

This project is dual-licensed under the [MIT](LICENSE_MIT.md) or [Apache License](LICENSE_APACHE.md).

## Contact

If you have any questions, need further assistance, you can contact me at `@winlogon.exe:matrix.org`.
