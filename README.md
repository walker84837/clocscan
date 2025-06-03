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

After installing, you can use clocscan to get the lines of code in your project. This is the usage for most things you'll need:

```console
./clocscan '/path/to/directory/'
```

This is the `--help` menu in case you need more specific options:

```
Usage: clocscan [OPTIONS] [WORK_FOLDER]

Arguments:
  [WORK_FOLDER]  The folder where the lines of code will be counted [default: .]

Options:
  -c, --config <CONFIG>    The JSON config file for code file extensions and ignore rules [default: config.json]
  -v, --verbose...         Use logging (-v for warn, -vv for debug logging, or none to only print errors)
  -s, --show-time-elapsed  Show how much time it took to count the lines of code
  -h, --help               Print help
```

## Speed

As a benchmark, I did put it up against [commit 7f9039c52](https://github.com/torvalds/linux/commit/7f9039c524a351c684149ecf1b3c5145a0dff2fe) of the Linux kernel.
I ran this test on an AMD Ryzen 5 7600, Samsung 990 PRO M.2 SSD with 32 GB of RAM.

```
clocscan .  5.90s user 9.66s system 451% cpu 3.443 total
cloc .  71.28s user 1.99s system 99% cpu 1:13.57 total
```
Due to asynchronous directory traversal, which uses more than one CPU core, I noticed that it's much faster than [`cloc`](https://github.com/AlDanial/cloc). I'm not sure how accurate the `time` command is, but this shows the difference in speed.
However, this may be because it's simpler than cloc, or because my code might not be very efficient.

## Contributing

Contributiions are always welcome! Feel free to propose changes to improve performance, or add some features.

If you feel like contributing and want to explore the code:
- start from the [roadmap](#roadmap)
- to look at the structure, docs are at <https://walker84837.github.io/clocscan/>. I try my best to document changes.

### Roadmap

- [ ] Add more file types in the example config file
- [ ] Make configuration types configurable in the JSON files
- [ ] Add an option to just output the total lines of code and more options (for scripting purposes)

## License

This project is dual-licensed under the [MIT](LICENSE_MIT.md) and [Apache License](LICENSE_APACHE.md), either at your choice.

## Contact

If you have any questions, need further assistance, you can contact me at `@winlogon.exe:matrix.org`.
