//! VSCode auto detection: Respect `json.schemas` field at `.vscode/settings.json` if present
// https://code.visualstudio.com/docs/languages/json#_json-schemas-and-settings

use super::{Feature, Schema};
use crate::regularize;
use globwalk::GlobWalkerBuilder;
use jsonc_parser::parse_to_serde_value;
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

pub struct Vscode;

fn read_schema_associations_from_settings() -> Option<Vec<Value>> {
    let settings_json = Path::new(".vscode/settings.json");
    if !settings_json.exists() {
        eprintln!("No .vscode/settings.json found");
        return None;
    }
    let Ok(settings_text) = fs::read_to_string(settings_json) else {
        eprintln!("Failed to read .vscode/settings.json");
        return None;
    };
    let Ok(Some(settings)) = parse_to_serde_value(&settings_text, &Default::default()) else {
        eprintln!("Failed to parse .vscode/settings.json");
        return None;
    };
    let Some(schema_settings) = settings.get("json.schemas") else {
        eprintln!("`json.schemas` field not found in .vscode/settings.json");
        return None;
    };
    let Value::Array(association_definitions) = schema_settings else {
        eprintln!("`json.schemas` field is not an array");
        return None;
    };
    Some(association_definitions.to_vec())
}

impl Feature for Vscode {
    fn get_associations(&self) -> HashMap<Schema, HashSet<PathBuf>> {
        let Ok(working_dir) = Path::new(".").canonicalize() else {
            eprintln!("Failed to canonicalize working directory");
            return HashMap::new();
        };
        let Some(association_definitions) = read_schema_associations_from_settings() else {
            eprintln!("Failed to get json.schemas as array");
            return HashMap::new();
        };
        let mut associations = HashMap::new();
        for association_definition in association_definitions {
            // Unwrap the association object
            let Value::Object(schema) = association_definition else {
                eprintln!("Schema is not an object");
                continue;
            };

            // Unwrap the `url` field (schema path)
            let Some(schema_path) = schema.get("url") else {
                eprintln!("`url` field not found in schema");
                continue;
            };
            let Value::String(schema_path) = schema_path else {
                eprintln!("`url` field is not a string");
                continue;
            };
            if schema_path.starts_with("http://") || schema_path.starts_with("https://") {
                eprintln!("Remote schema is not supported");
                continue;
            }
            let mut schema_path = schema_path.to_string();
            // Resolve schema paths
            if schema_path.starts_with('/') {
                // Relative to workspace root
                schema_path.remove(0); // Remove leading `/`
            }
            let schema_path = Path::new(&schema_path);
            let Ok(schema_path) = schema_path.canonicalize() else {
                eprintln!(
                    "Failed to canonicalize schema path `{}`",
                    schema_path.to_string_lossy()
                );
                continue;
            };
            let schema_path = regularize(&working_dir, &schema_path);

            // Unwrap the `fileMatch` field (array of glob patterns)
            let Some(file_match) = schema.get("fileMatch") else {
                eprintln!("`fileMatch` field not found in schema");
                continue;
            };
            let Value::Array(file_match) = file_match else {
                eprintln!("`fileMatch` field is not an array");
                continue;
            };
            let patterns = file_match
                .iter()
                .filter_map(|pattern| {
                    let Value::String(pattern) = pattern else {
                        eprintln!("`fileMatch` field contains non-string element");
                        return None;
                    };
                    Some(pattern.to_string())
                })
                .collect::<Vec<_>>();

            // Create a GlobWalker for given patterns
            let builder = GlobWalkerBuilder::from_patterns(".", &patterns);

            // Collect instances
            let instances = builder
                .build()
                .expect("Failed to build GlobWalker")
                .filter_map(|item| {
                    let Ok(item) = item else {
                        eprintln!("Failed to read item: {}", item.unwrap_err());
                        return None;
                    };
                    let Ok(path) = item.path().canonicalize() else {
                        eprintln!(
                            "Failed to canonicalize instance path `{}`",
                            item.path().to_string_lossy()
                        );
                        return None;
                    };
                    Some(regularize(&working_dir, &path))
                })
                .collect::<HashSet<_>>();

            // Update associations
            let schema = Schema::Local(schema_path);
            associations
                .entry(schema)
                .or_insert_with(HashSet::new)
                .extend(instances);
        }
        return associations;
    }
}
