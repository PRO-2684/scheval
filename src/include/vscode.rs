//! VSCode auto detection: Respect `json.schemas` field at `.vscode/settings.json` if present
// https://code.visualstudio.com/docs/languages/json#_json-schemas-and-settings

use super::{Include, Schema};
use crate::regularize;
use globwalk::GlobWalkerBuilder;
use jsonc_parser::parse_to_serde_value;
use serde_json::{Value, Map};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

/// A smart including feature of scheval, capable of respecting `json.schemas` field at `.vscode/settings.json` if present.
pub struct Vscode {
    /// Canonicalized path to the base directory.
    base: PathBuf,
}

fn read_schema_associations_from_settings(base: &Path) -> Option<Vec<Value>> {
    let settings_json = base.join(".vscode/settings.json");
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

/// Get schema from an association definition relative to `base`, **consuming** the definition
fn get_schema(mut association_definition: Map<String, Value>, base: &Path) -> Option<Schema>  {
    // Unwrap the `url` or `schema` field (schema path or inline schema)
    let Some(schema_path) = association_definition.get("url") else {
        // If `url` field is not found, try `schema` field
        let Some(schema) = association_definition.remove("schema") else {
            eprintln!("Neither `url` nor `schema` field found in schema");
            return None;
        };
        let Value::Object(_) = schema else { // Check if `schema` is an object
            eprintln!("`schema` field is not an object");
            return None;
        };
        return Some(Schema::Inline(schema));
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
    if schema_path.starts_with('/') {
        // Relative to workspace root
        schema_path.remove(0); // Remove leading `/`
    }
    let schema_path = base.join(schema_path);
    let Ok(schema_path) = schema_path.canonicalize() else {
        eprintln!(
            "Failed to canonicalize schema path `{}`",
            schema_path.to_string_lossy()
        );
        return None;
    };
    let schema_path = regularize(&base, &schema_path);
    Some(Schema::Local(schema_path))
}

impl Include for Vscode {
    fn with_base(base: &str) -> Self {
        let base = Path::new(base).canonicalize().expect("Failed to canonicalize base directory");
        Self { base }
    }
    fn get_associations(&self) -> HashMap<Schema, HashSet<PathBuf>> {
        let base = &self.base;
        let Some(association_definitions) = read_schema_associations_from_settings(base) else {
            return HashMap::new();
        };
        let mut associations = HashMap::new();
        for association_definition in association_definitions {
            // Unwrap the association object
            let Value::Object(association_definition) = association_definition else {
                eprintln!("A non-object element found under `json.schemas`");
                continue;
            };

            // Unwrap the `fileMatch` field (array of glob patterns)
            let Some(file_match) = association_definition.get("fileMatch") else {
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
                    let mut pattern = pattern.to_string();
                    // Quick fix and warning for https://github.com/Gilnaa/globwalk/issues/28
                    if pattern.starts_with("./") {
                        pattern = pattern[2..].to_string();
                    } else if pattern.starts_with("../") {
                        eprintln!("`fileMatch` patterns starting with `../` are not supported");
                        return None;
                    }
                    Some(pattern)
                })
                .collect::<Vec<_>>();

            // Unwrap the `url` or `schema` field (schema path or inline schema)
            let Some(schema) = get_schema(association_definition, &base) else {
                eprintln!("Failed to get schema from association definition");
                continue;
            };

            // Create a GlobWalker for given patterns
            let builder = GlobWalkerBuilder::from_patterns(base, &patterns);

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
                    Some(regularize(&base, &path))
                })
                .collect::<HashSet<_>>();

            // Update associations
            associations
                .entry(schema)
                .or_insert_with(HashSet::new)
                .extend(instances);
        }
        return associations;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{TEST_DIR, hashset_of_pathbuf};

    #[test]
    fn test_vscode() {
        let inc = Vscode::with_base(TEST_DIR);
        let associations = inc.get_associations();
        let expected: HashMap<Schema, HashSet<PathBuf>> = [
            (
                Schema::Local(PathBuf::from("foo_schema.json")),
                hashset_of_pathbuf(&["nested1/nested2/abc.foo.json"]),
            ),
            (
                Schema::Inline(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "The name of the entry"
                        }
                    }
                })),
                hashset_of_pathbuf(&[".myconfig"]),
            ),
            (
                Schema::Local(PathBuf::from("receipts.schema.json")),
                hashset_of_pathbuf(&["receipts/1.json", "receipts/2.json"]),
            )
        ].into();
        assert_eq!(associations, expected);
    }
}
