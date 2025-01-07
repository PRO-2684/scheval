//! `scheval`: A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate.
//!
//! ## Command-Line Tool
//!
//! Note that if you see this message, you're viewing the documentation of the `scheval` **library crate**. To install the `scheval` **command-line tool** from source, you can run:
//!
//! ```shell
//! cargo install scheval
//! ```
//!
//! For more information and alternative installation methods for the CLI, please refer to the README file on one of the following platforms:
//!
//! - [GitHub](https://github.com/PRO-2684/scheval)
//! - [Crates.io](https://crates.io/crates/scheval)
//!
//! ## Library Crate
//!
//! The `scheval` library crate is designed to be a reusable and extensible library for smart JSON Schema validation. Built on top of the `jsonschema` crate, `scheval` does not implement JSON Schema validation itself. Rather, it provides a set of smart including features to automatically associate JSON instances with their corresponding schemas, and then validate them in batch.
//!
//! The main entry point of the library is the [`run`] function, which takes a [`Config`] and a base directory as input, and returns a `Result`.
//!
//! - The [`Config`] struct offers a simple way to configure what smart including features to use, and can be constructed either manually or automatically from command line arguments using the [`get_config`] function.
//! - The base directory is a string slice representing the base path, or working directory, from which we start to search for JSON instances and schemas and resolve relative paths.
//! - The return value is a `Result`, where:
//!     - `Ok(true)` indicates that all instances are valid.
//!     - `Ok(false)` indicates that at least one instance is invalid, or a invalid schema is encountered.
//!     - `Err(error)` indicates that an error occurred during the validation process. It could be an I/O error, a JSON parsing error etc.
//!
//! Refer to the binary crate for a complete example of using the `scheval` library crate.

pub mod include;
use clap::{
    builder::styling::{AnsiColor, Color, Style, Styles},
    Parser,
};
use include::Include;
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

// Arguments & Configuration

/// Command-line arguments.
#[derive(Parser, Debug)]
#[command(version, about = format!("A fast and {ITALIC}*smart*{ITALIC:#} command-line tool for JSON Schema validation, powered by the {UNDERLINE}`jsonschema`{UNDERLINE:#} crate."), long_about = None, styles = CLAP_STYLE)]
struct Args {
    /// What smart including features to use. Available: `vscode`, `suffix`. Default to all
    ///
    /// - `vscode`: Respect `json.schemas` field at `.vscode/settings.json` if present
    /// - `suffix`: Validate `<filename>.json` with `<filename>.schema.json` under working directory
    #[arg(short, long, verbatim_doc_comment)]
    include: Vec<String>,
    // /// What smart excluding features to use. Available: TBD
    // #[arg(short, long)]
    // exclude: Vec<String>,
}

/// Configuration options. (Simple wrapper around `Args`)
#[derive(Debug)]
pub struct Config {
    pub vscode: bool,
    pub suffix: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let all = args.include.is_empty();
        let vscode = args.include.contains(&"vscode".to_string()) || all;
        let suffix = args.include.contains(&"suffix".to_string()) || all;
        Self { vscode, suffix }
    }
}

/// Parse command line arguments and return configuration options.
pub fn get_config() -> Config {
    let args = Args::parse();
    args.into()
}

// JSON Schema Validation

/// A JSON Schema.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Schema {
    Local(PathBuf),
    Remote(String),
    Inline(Value),
}

impl Schema {
    /// Resolve the schema to a JSON value, **consuming `self`**.
    fn resolve(self, base: &Path) -> Result<Value, Box<dyn Error>> {
        use Schema::*;
        match self {
            Local(path) => {
                let path = base.join(path);
                let json = read_json(&path)??;
                Ok(json)
            }
            Remote(_) => Err("Remote schema is not supported yet".into()),
            Inline(value) => Ok(value),
        }
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(path) => write!(f, "{}", path.to_string_lossy()),
            Self::Remote(url) => write!(f, "{}", url),
            Self::Inline(_) => write!(f, "<inline schema>"),
        }
    }
}

/// Read JSON file from given path.
fn read_json(path: &Path) -> Result<serde_json::Result<Value>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader))
}

/// Validate a JSON instance against a JSON Schema.
pub fn validate_instance(
    validator: &jsonschema::Validator,
    instance: &Path,
) -> Result<bool, Box<dyn Error>> {
    let mut success = true;
    let instance_json = read_json(instance)??;
    let mut errors = validator.iter_errors(&instance_json);
    let filename = instance.to_string_lossy();
    if let Some(first) = errors.next() {
        success = false;
        println!("- `{filename}` - {FAILURE}INVALID{FAILURE:#}. Errors:");
        println!("  1. {first}");
        for (i, error) in errors.enumerate() {
            println!("  {}. {error}", i + 2);
        }
    } else {
        println!("- `{filename}` - {SUCCESS}VALID{SUCCESS:#}");
    }
    Ok(success)
}

// Helper Functions

/// Try to relativize `target` path, resort to absolute path if failed. Note that given paths must be canonicalized.
fn regularize(base: &Path, target: &Path) -> PathBuf {
    target
        .strip_prefix(base)
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|_| target.to_path_buf())
}

/// Extend `associations` with `new_associations`, **consuming `new_associations`**.
fn extend(
    associations: &mut HashMap<Schema, HashSet<PathBuf>>,
    new_associations: HashMap<Schema, HashSet<PathBuf>>,
) {
    for (schema, instances) in new_associations {
        associations
            .entry(schema)
            .or_insert_with(HashSet::new)
            .extend(instances);
    }
}

// Main Logic

/// Run scheval with given configuration.
pub fn run(config: &Config, base: &str) -> Result<bool, Box<dyn Error>> {
    let mut success = true;
    let mut associations = HashMap::new();
    if config.vscode {
        let inc = include::Vscode::with_base(base);
        let vscode_associations = inc.get_associations();
        extend(&mut associations, vscode_associations);
    }
    if config.suffix {
        let inc = include::Suffix::with_base(base);
        let suffix_associations = inc.get_associations();
        extend(&mut associations, suffix_associations);
    }
    let base = Path::new(base);
    for (schema, instances) in associations {
        println!("Schema `{schema}`:");
        let schema_json = schema.resolve(base)?;
        let validator = match jsonschema::validator_for(&schema_json) {
            Ok(validator) => validator,
            Err(error) => {
                println!("{FAILURE}Invalid schema{FAILURE:#}: {error}\n");
                success = false;
                continue;
            }
        };
        for instance in instances {
            let instance = base.join(instance);
            let valid = validate_instance(&validator, &instance)?;
            success &= valid;
        }
        println!();
    }
    Ok(success)
}

#[cfg(test)]
pub(crate) mod tests_util {
    use std::path::PathBuf;

    /// Public constant for the path to the test data directory.
    pub const TEST_DIR: &str = "tests/data";

    /// Create a hashset of PathBuf from a list of paths.
    pub fn hashset_of_pathbuf(paths: &[&str]) -> std::collections::HashSet<PathBuf> {
        paths.iter().map(|p| PathBuf::from(p)).collect()
    }
}

// Styling

// Colors for success and failure messages
/// ANSI color for green
const GREEN: Color = Color::Ansi(AnsiColor::Green);
/// ANSI color for red
const RED: Color = Color::Ansi(AnsiColor::Red);
/// Style for success messages
const SUCCESS: Style = Style::new().fg_color(Some(GREEN)).bold();
/// Style for failure messages
const FAILURE: Style = Style::new().fg_color(Some(RED)).bold();

/// Styling for clap help messages
// Adapted from https://github.com/8LWXpg/ptr/blob/83aa1d1814ec98d7854e1f4df52d66b8172f6eda/src/main.rs#L124-L131
const CLAP_STYLE: Styles = clap::builder::Styles::styled()
    .usage(AnsiColor::BrightGreen.on_default())
    .header(AnsiColor::BrightGreen.on_default())
    .literal(AnsiColor::BrightCyan.on_default())
    .invalid(AnsiColor::BrightYellow.on_default())
    .error(AnsiColor::BrightRed.on_default().bold())
    .valid(AnsiColor::BrightGreen.on_default())
    .placeholder(AnsiColor::Cyan.on_default());
/// Italic style
const ITALIC: Style = Style::new().italic();
/// Dotted-underline style
const UNDERLINE: Style = Style::new().effects(clap::builder::styling::Effects::DOTTED_UNDERLINE);
