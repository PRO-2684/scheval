use clap::Parser;
use std::{fs::File, io::BufReader, path::Path};

mod features;
use features::Feature;

/// Command line arguments.
#[derive(Parser, Debug)]
#[command(version, about = "A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate.", long_about = None)]
struct Args {
    /// Enable VSCode auto detection: Respect `json.schemas` field at `.vscode/settings.json` if present.
    #[arg(short, long)]
    vscode: bool,
    /// Enable suffix auto detection: Validate `<filename>.json` with `<filename>.schema.json` under working directory.
    #[arg(short, long)]
    suffix: bool,
    /// Enable all auto detection features.
    #[arg(short, long, conflicts_with_all = ["vscode", "suffix"])]
    all: bool,
}

/// Configuration options. (Simple wrapper around `Args`)
#[derive(Debug)]
pub struct Config {
    pub vscode: bool,
    pub suffix: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let vscode = args.vscode || args.all;
        let suffix = args.suffix || args.all;
        Self { vscode, suffix }
    }
}

/// Parse command line arguments and return configuration options.
pub fn get_config() -> Config {
    let args = Args::parse();
    args.into()
}

/// Read JSON file from given path.
fn read_json(
    path: &Path,
) -> Result<serde_json::Result<serde_json::Value>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader))
}

/// Validate a JSON instance against a JSON Schema.
pub fn validate_instance(
    instance: &Path,
    schema_path: &Path,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut success = true;

    let schema_json = read_json(schema_path)??;
    match jsonschema::validator_for(&schema_json) {
        Ok(validator) => {
            let instance_json = read_json(instance)??;
            let mut errors = validator.iter_errors(&instance_json);
            let filename = instance.to_string_lossy();
            if let Some(first) = errors.next() {
                success = false;
                println!("{filename} - INVALID. Errors:");
                println!("1. {first}");
                for (i, error) in errors.enumerate() {
                    println!("{}. {error}", i + 2);
                }
            } else {
                println!("{filename} - VALID");
            }
        }
        Err(error) => {
            println!("Schema is invalid. Error: {error}");
            success = false;
        }
    }
    Ok(success)
}

/// Run scheval with given configuration.
pub fn run(config: &Config) -> Result<bool, Box<dyn std::error::Error>> {
    let mut success = true;
    if config.vscode {
        // Dummy
    }
    if config.suffix {
        let feature = features::Suffix;
        let instances = feature.get_instances().collect::<Vec<_>>();
        for (schema_path, instance_path) in instances {
            success &= validate_instance(&Path::new(&instance_path), &Path::new(&schema_path))?;
        }
    }
    Ok(success)
}
