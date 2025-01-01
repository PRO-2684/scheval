//! VSCode auto detection: Respect `json.schemas` field at `.vscode/settings.json` if present
// https://code.visualstudio.com/docs/languages/json#_json-schemas-and-settings

use super::Feature;
// use glob::glob;
use globwalk::GlobWalkerBuilder;
use jsonc_parser::parse_to_serde_value;
use serde_json::Value;
use std::{fs, path::Path};

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
    let Value::Array(schemas) = schema_settings else {
        eprintln!("`json.schemas` field is not an array");
        return None;
    };
    Some(schemas.to_vec())
}

impl Feature for Vscode {
    fn get_instances(&self) -> impl Iterator<Item = (String, String)> {
        let associations = read_schema_associations_from_settings().expect("Failed to get json.schemas as array");
        associations
            .into_iter()
            .filter_map(|association| {
                // Unwrap the association object
                let Value::Object(schema) = association else {
                    eprintln!("Schema is not an object");
                    return None;
                };

                // Unwrap the `url` field (schema path)
                let Some(schema_path) = schema.get("url") else {
                    eprintln!("`url` field not found in schema");
                    return None;
                };
                let Value::String(schema_path) = schema_path else {
                    eprintln!("`url` field is not a string");
                    return None;
                };
                if schema_path.starts_with("http://") || schema_path.starts_with("https://") {
                    eprintln!("Remote schema is not supported");
                    return None;
                }
                let mut schema_path = schema_path.to_string();
                // Resolve schema paths
                if schema_path.starts_with('/') { // Relative to workspace root
                    schema_path.remove(0); // Remove leading `/`
                } else { // Relative to `.vscode` directory
                    schema_path.insert_str(0, ".vscode/"); // Prepend `.vscode/`
                }

                // Unwrap the `fileMatch` field (array of glob patterns)
                let Some(file_match) = schema.get("fileMatch") else {
                    eprintln!("`fileMatch` field not found in schema");
                    return None;
                };
                let Value::Array(file_match) = file_match else {
                    eprintln!("`fileMatch` field is not an array");
                    return None;
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

                // Return instances as a tuple of schema path and instance path
                let instances = builder
                    .build()
                    .expect("Failed to build GlobWalker")
                    .filter_map(Result::ok)
                    .filter_map(move |instance_path| {
                        let instance_path = instance_path.path().to_str().unwrap().to_string();
                        Some((schema_path.clone(), instance_path))
                    });
                Some(instances)
            })
            .flatten()
    }
}
