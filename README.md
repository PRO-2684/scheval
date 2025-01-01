# scheval

A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate. Currently still in development.

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

  -e, --exclude <EXCLUDE>
          What smart excluding features to use. Available: TBD

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Note that `vscode` are not supported yet.

## Notes

This project uses [`globwalk`](https://github.com/Gilnaa/globwalk) for pattern matching, which might be buggy. Notably in `fileMatch` field:

- `./` is not supported (I've included a quick fix by removing the prefix, but not considering other cases, like `././` or `!./`)
- `../` is not supported (A warning will be shown, and the pattern will be ignored)

If you have a good alternative which supports [VSCode's `fileMatch` syntax](https://code.visualstudio.com/docs/languages/json#_file-match-syntax), please let me know.

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
  - [ ] Respect `.gitignore`
  - [ ] Ignore paths starting with `.` (hidden files)
- [ ] Add more tests & documentation
- [ ] Better error handling
- [ ] Handle output in `main.rs`
- [ ] Improve performance using references
- [ ] Possibly adding more features
- [ ] [Minimize binary size](https://github.com/johnthagen/min-sized-rust)
- [ ] Auto release and publish using GitHub Actions
