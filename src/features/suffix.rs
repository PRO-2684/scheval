//! Suffix auto detection: Validate `<filename>.json` with `<filename>.schema.json` under working directory.

use super::{Feature, Schema};
use crate::regularize;
use std::{collections::{HashMap, HashSet}, fs, path::{Path, PathBuf}};

/// A feature of scheval, capable of finding `<filename>.json` with `<filename>.schema.json` under working directory.
pub struct Suffix;

impl Feature for Suffix {
    fn get_associations(&self) -> HashMap<Schema, HashSet<PathBuf>> {
        let Ok(working_dir) = Path::new(".").canonicalize() else {
            eprintln!("Failed to canonicalize working directory");
            return HashMap::new();
        };
        let Ok(entries) = fs::read_dir(&working_dir) else {
            eprintln!("Failed to list working directory");
            return HashMap::new();
        };
        let mut associations = HashMap::new();
        for entry in entries {
            let Ok(entry) = entry else {
                eprintln!("Failed to read entry: {}", entry.unwrap_err());
                continue;
            };
            let path = entry.path();
            let Some(file_name) = path.file_name() else {
                eprintln!("Failed to get file name");
                continue;
            };
            let Some(file_name) = file_name.to_str() else {
                eprintln!("Failed to convert file name to string");
                continue;
            };
            if file_name.ends_with(".json") {
                let schema_path = path.with_extension("schema.json");
                if schema_path.exists() {
                    let Ok(schema_path) = schema_path.canonicalize() else {
                        eprintln!("Failed to canonicalize schema path `{}`", schema_path.to_string_lossy());
                        continue;
                    };
                    let schema_path = regularize(&working_dir, &schema_path);
                    let schema = Schema::Local(schema_path);
                    let Ok(instance) = path.canonicalize() else {
                        eprintln!("Failed to canonicalize instance path `{}`", path.to_string_lossy());
                        continue;
                    };
                    let instance = regularize(&working_dir, &instance);
                    associations.entry(schema).or_insert_with(HashSet::new).insert(instance);
                }
            }
        }
        associations
    }
}
