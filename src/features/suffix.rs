//! suffix auto detection: Validate `<filename>.json` with `<filename>.schema.json` under working directory.

use super::Feature;
use std::{fs, path::Path};

/// A feature of scheval, capable of finding `<filename>.json` with `<filename>.schema.json` under working directory.
pub struct Suffix;

impl Feature for Suffix {
    fn get_instances(&self) -> impl Iterator<Item = (String, String)> {
        let working_dir = Path::new(".");
        let entries = fs::read_dir(&working_dir).expect("Failed to list working directory");
        entries.filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".json") {
                let schema_path = path.with_extension("schema.json");
                if schema_path.exists() {
                    let instance_path = path.to_str().unwrap().to_string();
                    let schema_path = schema_path.to_str().unwrap().to_string();
                    Some((schema_path, instance_path))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}
