#![allow(clippy::print_stdout)]
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use clap::Parser;

mod features;

use features::Feature;

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

pub fn get_config() -> Config {
    let args = Args::parse();
    args.into()
}

fn read_json(
    path: &Path,
) -> Result<serde_json::Result<serde_json::Value>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader))
}

pub fn validate_instance(
    instance: &PathBuf,
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

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    if config.vscode {
        // Dummy
    }
    if config.suffix {
        let feature = features::Suffix;
        let instances = feature.get_instances().collect::<Vec<_>>();
        for (schema_path, instance_path) in instances {
            validate_instance(&PathBuf::from(&instance_path), &PathBuf::from(&schema_path))?;
        }
    }
    Ok(())
}