# ClocScan: counter of lines of code

[![Rust](https://github.com/walker84837/clocscan/actions/workflows/rust.yml/badge.svg)](https://github.com/walker84837/clocscan/actions/workflows/rust.yml)

![clocscan](https://github.com/user-attachments/assets/508aab11-1600-452c-87ef-c4aebae35dd9)

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

## Speed

I did some tests to check the speed, and I saw that due to asynchronous directory traversal, it's way faster than [`cloc`]().

Running a directory traversal on [commit 7f9039c52](https://github.com/torvalds/linux/commit/7f9039c524a351c684149ecf1b3c5145a0dff2fe) the Linux kernel.

Running on an AMD Ryzen 5 7600, Samsung 990 PRO M.2 SSD with 32 GB of RAM.

```
clocscan .  5.90s user 9.66s system 451% cpu 3.443 total
cloc .  71.28s user 1.99s system 99% cpu 1:13.57 total
```
Notice how clocscan uses more than one CPU core because of the async traversal.

I'm not really sure how accurate the `time` command is, but this should make it "obvious" how faster it is.

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
