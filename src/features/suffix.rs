//! Suffix auto detection: Validate `<filename>.json` with `<filename>.schema.json` under working directory.

use super::{Feature, Schema};
use crate::regularize;
use std::{collections::{HashMap, HashSet}, fs, path::{Path, PathBuf}};

/// A feature of scheval, capable of finding `<filename>.json` with `<filename>.schema.json` under base directory.
pub struct Suffix {
    /// Canonicalized path to the base directory.
    base: PathBuf,
}

impl Feature for Suffix {
    fn with_base(base: &str) -> Self {
        let base = Path::new(base).canonicalize().expect("Failed to canonicalize base directory");
        Self { base }
    }
    fn get_associations(&self) -> HashMap<Schema, HashSet<PathBuf>> {
        let base = &self.base;
        let Ok(entries) = fs::read_dir(&base) else {
            eprintln!("Failed to list base directory");
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
                    let schema_path = regularize(&base, &schema_path);
                    let schema = Schema::Local(schema_path);
                    let Ok(instance) = path.canonicalize() else {
                        eprintln!("Failed to canonicalize instance path `{}`", path.to_string_lossy());
                        continue;
                    };
                    let instance = regularize(&base, &instance);
                    associations.entry(schema).or_insert_with(HashSet::new).insert(instance);
                }
            }
        }
        associations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{TEST_DIR, hashset_of_pathbuf};

    #[test]
    fn test_suffix() {
        let feature = Suffix::with_base(TEST_DIR);
        let associations = feature.get_associations();
        let expected: HashMap<Schema, HashSet<PathBuf>> = [
            (
                Schema::Local(PathBuf::from("receipts.schema.json")),
                hashset_of_pathbuf(&["receipts.json"]),
            )
        ].into();
        assert_eq!(associations, expected);
    }
}
