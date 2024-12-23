#![allow(clippy::print_stdout)]
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about = "A fast and *smart* command-line tool for JSON Schema validation, powered by the `jsonschema` crate.", long_about = None)]
pub struct Args {
    /// Enable VSCode auto detection: Respect `json.schemas` field at `.vscode/settings.json` if present.
    #[arg(short, long)]
    vscode: bool,
    /// Enable suffix auto detection: Validate `<filename>.json` with `<filename>.schema.json`.
    #[arg(short, long)]
    suffix: bool,
    /// Enable all auto detection features.
    #[arg(short, long, conflicts_with_all = ["vscode", "suffix"])]
    all: bool,
}

fn read_json(
    path: &Path,
) -> Result<serde_json::Result<serde_json::Value>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader))
}

pub fn validate_instances(
    instances: &[PathBuf],
    schema_path: &Path,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut success = true;

    let schema_json = read_json(schema_path)??;
    match jsonschema::validator_for(&schema_json) {
        Ok(validator) => {
            for instance in instances {
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
        }
        Err(error) => {
            println!("Schema is invalid. Error: {error}");
            success = false;
        }
    }
    Ok(success)
}

pub fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    // Dummy implementation
    println!("{:?}", args);
    Ok(())
}
