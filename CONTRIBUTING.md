# Contributing to clocscan

Thanks for your interest in contributing!

This project is under heavy development, so contributions that improve performance, add new features, or clean up code are very welcome :D

## Submitting a PR (general overview)

1. Fork the repo and create a feature branch.
2. Format and lint your code.
3. Ensure the commit messages follow the Conventional Commits spec.
4. Run tests locally.
5. Open a PR with a clear title and description:
   * What changed and why?
   * Whether the changes have been tested.
   * Benchmark results (if performance-related).

## Formatting and committing your code

Before opening a PR, make sure your code is properly formatted and linted.

1. Formatting code for a consistent code style across the project:
   ```bash
   cargo fmt
   ```

2. Lint code:
   ```bash
   cargo clippy -- -D warnings
   ```
   We treat warnings as errors. If Clippy suggests fixes, apply them unless there's a clear reason not to.

## Committing your changes

### Commit Guidelines

We use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

So, each commit message should:

* Start with a type (e.g. `feat`, `fix`, `refactor`, `chore`, `docs`, `test`, `perf`).
* Be written in the *imperative* mood ("add X", **not** "added X").
* Keep the subject **under 70 characters** (50 is a soft imit).

Example:

```
feat: add asynchronous directory traversal
fix: handle missing config file gracefully
perf: improve SLOC counting performance
```

If your commit affects a specific module or file, you can scope it:

```
feat(config): add support for YAML config
```

### Branching

Always branch off `main` using a fairly short, descriptive name:

| Type     | Example                   |
| -------- | ------------------------- |
| Feature  | `feat/add-something-cool` |
| Fix      | `fix/handle-empty-paths`  |
| Refactor | `refactor/config-loader`  |
| Docs     | `docs/update-readme`      |

Before merging:

1. Rebase your branch onto the latest `main`.
2. Run tests and lint checks locally.
3. Ensure all CI checks pass.

## Testing Changes

If you add or modify features:

* Include unit tests when possible.
* Run all tests with:

  ```bash
  cargo test
  ```
* For performance-related changes, include before/after benchmarks.


## Developer Tips

- **Run with more logging**:  
  ```bash
  ./clocscan -vv   # info output
  ./clocscan -vvv  # debug output

- **Quick output**:
  ```bash
  ./clocscan --sloc-only .
  ```

- **Performance benchmarking**:
  I use [`hyperfine`](https://github.com/sharkdp/hyperfine) for consistent measurements.
  To compare performance against `cloc`, run:

  ```bash
  hyperfine --warmup 3 --runs 10 \
    --export-markdown benchmark_results.md \
    './target/release/clocscan linux/' 'cloc linux/'
  ```

  If you're working on performance improvements:
  1. Clone the Linux kernel at the commit used for reference (`8bb886cb8f3a2811430ddb7d9838e245c57e7f7c`; it might be changed in the future to the latest version).
  2. Run benchmarks before and after your change.
  3. Include your results (the contents of `benchmark_results.md`) in the PR description.
