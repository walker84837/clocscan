# clocscan: counter of lines of code

[![Rust](https://github.com/walker84837/clocscan/actions/workflows/rust.yml/badge.svg)](https://github.com/walker84837/clocscan/actions/workflows/rust.yml)

![clocscan showcase: compiling and running](https://github.com/user-attachments/assets/508aab11-1600-452c-87ef-c4aebae35dd9)

ClocScan is a flexible and blazingly fast counter of lines of code. See [sample.json](sample.json) for an example configuration with supported file types and ignore rules.

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

These are the other options in case you need fine-grained outputs:

- `-c, --config (config)` (`string`): The JSON config file for code file extensions and ignore rules (default: `config.json`)
- Verbose (flag). Valid options are:
  - `-v` for warn
  - `-vv` for info
  - `-vvv` for debug
  - Default only logs errors
- `-s, --show-time-elapsed` (flag): Show how much time it took to count the lines of code
- `-h, --help` (flag): Print help menu

## Speed

As a benchmark, I put it up against [commit 8bb886cb8](https://github.com/torvalds/linux/commit/8bb886cb8f3a2811430ddb7d9838e245c57e7f7c) of the Linux kernel.

I ran this test on an AMD Ryzen 5 7600, Samsung 990 PRO M.2 SSD with 32 GB of RAM. With that out of the way, here's the results:

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/clocscan linux/` | 5.513 ± 0.021 | 5.476 | 5.538 | 1.00 |
| `cloc linux/` | 78.272 ± 0.832 | 77.115 | 80.084 | 14.20 ± 0.16 |

These benchmarks were run using `hyperfine`[^1] for more accurate results. `clocscan` uses asynchronous directory traversal, which uses multiple CPU cores, making it significantly faster performance compared to [`cloc`](https://github.com/AlDanial/cloc), but this could be improved further.

## Contributing

Contributions are always welcome! Feel free to propose changes to improve performance, or add some features.

If you feel like contributing and want to explore the code:
- start from the [roadmap](#roadmap)
- look at the structure, this repo hosts Rust docs at <https://walker84837.github.io/clocscan/>. I document new code where it makes sense the most.

### Roadmap

- [ ] Add more file types in the example config file
- [ ] Make configuration types configurable in the JSON files
- [ ] Add an option to just output the total lines of code and more options (for scripting purposes)

## License

This project is dual-licensed under the [MIT](LICENSE_MIT.md) and [Apache License](LICENSE_APACHE.md), either at your choice.

## Contact

If you have any questions, need further assistance, you can contact me at `@winlogon.exe:matrix.org`.

[^1]: The [hyperfine](https://github.com/sharkdp/hyperfine) command I used was `hyperfine --warmup 3 --runs 10 --export-markdown benchmark_results.md './target/release/clocscan linux/' 'cloc linux/'`
