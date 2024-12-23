# scheval

A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate. Currently still in development.

## Usage

```shell
$ scheval --help
A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate.

Usage: scheval [OPTIONS]

Options:
  -v, --vscode   Enable VSCode auto detection: Respect `json.schemas` field at `.vscode/settings.json` if present
  -s, --suffix   Enable suffix auto detection: Validate `<filename>.json` with `<filename>.schema.json` under working directory
  -a, --all      Enable all auto detection features
  -h, --help     Print help
  -V, --version  Print version
```

Note that `vscode` are not supported yet.
