# scheval

A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate. Currently still in development.

## Installation

<!-- TODO: Add `cargo-binstall` support -->
<!-- If you have `cargo-binstall`, you can install this tool by running:

```shell
$ cargo binstall scheval
```

Otherwise, you can install it using `cargo`: -->

```shell
$ cargo install scheval
```

## Features

### Including

- `vscode`: Respect `json.schemas` field at `.vscode/settings.json` if present
- `suffix`: Validate `<filename>.json` with `<filename>.schema.json` under working directory

### Excluding

TBD

## Quick Start

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

## Usage

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

## Notes

This project uses [`globwalk`](https://github.com/Gilnaa/globwalk) for pattern matching, which might be buggy. Notably in `fileMatch` field:

- `./` is not supported (I've included a quick fix by removing the prefix, but have not considered other cases, like `././` or `!./`)
- `../` is not supported (A warning will be shown, and the pattern will be ignored)

If you have a good alternative which supports [VSCode's `fileMatch` syntax](https://code.visualstudio.com/docs/languages/json#_file-match-syntax), please let me know.

Possible alternatives:

- [globset](https://docs.rs/globset/0.4.15/globset/)
- [ignore](https://docs.rs/ignore/latest/ignore/struct.WalkBuilder.html)

## TODO

- [ ] Implement `vscode` auto detection
  - [x] Better support for `fileMatch`
    - [x] Relative path
    - [x] Absolute path (workspace)
  - [ ] Better support for `url`
    - [x] Local schema (path)
    - [ ] Remote schema (URL)
  - [x] Support for `schema` (Inline schema)
- [ ] Smartly exclude paths
- [ ] Add more tests & documentation
- [ ] Better error handling
- [ ] Handle output in `main.rs`
- [ ] Improve performance using references
- [ ] Possibly adding more features
- [ ] [Minimize binary size](https://github.com/johnthagen/min-sized-rust)
- [ ] Automation using GitHub Actions
  - [ ] Release (respecting `cargo-binstall`)
  - [ ] Publish
