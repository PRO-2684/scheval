# scheval

[![GitHub License](https://img.shields.io/github/license/PRO-2684/scheval?logo=opensourceinitiative)](https://github.com/PRO-2684/scheval/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/scheval/release.yml?branch=main&logo=githubactions)](https://github.com/PRO-2684/scheval/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/scheval?logo=githubactions)](https://github.com/PRO-2684/scheval/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/scheval/total?logo=github)](https://github.com/PRO-2684/scheval/releases)
[![GitHub Downloads (all assets, latest release)](https://img.shields.io/github/downloads/PRO-2684/scheval/latest/total?logo=github)](https://github.com/PRO-2684/scheval/releases/latest)
[![Crates.io Version](https://img.shields.io/crates/v/scheval?logo=rust)](https://crates.io/crates/scheval)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/scheval?logo=rust)](https://crates.io/crates/scheval)

A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate. Currently still in development.

## 🚀 Installation

If you have `cargo-binstall`, you can install this tool by running:

```shell
$ cargo binstall scheval
```

Otherwise, you can install it from source:

```shell
$ cargo install scheval
```

## ✨ Features

### Including

- `vscode`: Respect `json.schemas` field at `.vscode/settings.json` if present
- `suffix`: Validate `<filename>.json` with `<filename>.schema.json` under working directory

### Excluding

TBD

## 🚀 Quick Start

`scheval` can be run without any arguments. It will automatically use all available features.

```shell
$ scheval
```

Alternatively, you can specify a list of smart including features to use:

```shell
$ scheval --include vscode # Uses only `vscode`
$ scheval --include suffix # Uses only `suffix`
$ scheval --include vscode --include suffix # Uses both `vscode` and `suffix`
$ # Shorthands provided by `clap`
$ scheval -i vscode -i suffix
$ scheval -ivscode -isuffix
```

## 📚 Usage

```shell
$ scheval --help
A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate.

Usage: scheval [OPTIONS]

Options:
  -i, --include <INCLUDE>
          What smart including features to use. Available: `vscode`, `suffix`. Default to all

          - `vscode`: Respect `json.schemas` field at `.vscode/settings.json` if present
          - `suffix`: Validate `<filename>.json` with `<filename>.schema.json` under working directory

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## 📝 Notes

This project uses [`globwalk`](https://github.com/Gilnaa/globwalk) for pattern matching, which might be buggy. Notably in `fileMatch` field:

- `./` is not supported (I've included a quick fix by removing the prefix, but have not considered other cases, like `././` or `!./`)
- `../` is not supported (A warning will be shown, and the pattern will be ignored)

If you have a good alternative which supports [VSCode's `fileMatch` syntax](https://code.visualstudio.com/docs/languages/json#_file-match-syntax), please let me know.

Possible alternatives:

- [globset](https://docs.rs/globset/)
- [ignore](https://docs.rs/ignore/)

## TODO

- [ ] Implement `vscode` auto detection
  - [x] Better support for `fileMatch`
    - [x] Relative path
    - [x] Absolute path (workspace)
  - [ ] Support for `url`
    - [x] Local schema (path)
    - [ ] Remote schema (URL)
  - [x] Support for `schema` (Inline schema)
- [ ] Smartly exclude paths
- [ ] Add more tests & documentation
- [ ] Better error handling
- [ ] Output
  - [ ] Handle output in `main.rs`
  - [ ] Use commandline argument `--verbose` for increased verbosity
  - [ ] Colorize output (https://docs.rs/anstyle or https://docs.rs/colored)
- [ ] Improve performance using references
- [ ] Possibly adding more features
- [x] [Reduce binary size](https://github.com/johnthagen/min-sized-rust)
- [x] Automation using GitHub Actions
  - [x] Release (respecting `cargo-binstall`)
  - [x] Publish
