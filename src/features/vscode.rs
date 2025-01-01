//! VSCode auto detection: Respect `json.schemas` field at `.vscode/settings.json` if present
// https://code.visualstudio.com/docs/languages/json#_json-schemas-and-settings

use super::Feature;
use glob::glob;
use jsonc_parser::parse_to_serde_value;
use serde_json::Value;
use std::{fs, path::Path};

pub struct Vscode;

fn read_schemas_from_settings() -> Option<Vec<Value>> {
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
        let schemas = read_schemas_from_settings().expect("Failed to get json.schemas as array");
        schemas
            .into_iter()
            .filter_map(|schema| {
                let Value::Object(schema) = schema else {
                    eprintln!("Schema is not an object");
                    return None;
                };
                let Some(file_match) = schema.get("fileMatch") else {
                    eprintln!("`fileMatch` field not found in schema");
                    return None;
                };
                let Value::String(file_match) = file_match else {
                    eprintln!("`fileMatch` field is not a string");
                    return None;
                };
                let Some(schema_path) = schema.get("url") else {
                    eprintln!("`url` field not found in schema");
                    return None;
                };
                let Value::String(schema_path) = schema_path else {
                    eprintln!("`url` field is not a string");
                    return None;
                };
                let schema_path = schema_path.to_string();
                let instances = glob(file_match)
                    .expect("Failed to glob fileMatch")
                    .filter_map(Result::ok)
                    .filter_map(move |instance_path| {
                        let instance_path = instance_path.to_str().unwrap().to_string();
                        Some((schema_path.clone(), instance_path))
                    });
                Some(instances)
            })
            .flatten()
    }
}
