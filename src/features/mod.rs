//! This module contains the feature trait and re-exports all features for convenience.

mod suffix;
mod vscode;
use crate::Schema;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
pub use suffix::Suffix;
pub use vscode::Vscode;

/// A feature of scheval that is capable of finding JSON instances and their corresponding schemas.
pub trait Feature {
    /// Create a new instance of the feature.
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::with_base(".")
    }
    /// Create a new instance of the feature with a base directory.
    fn with_base(base: &str) -> Self;
    /// Generate a map from JSON schema to paths to JSON instances.
    fn get_associations(&self) -> HashMap<Schema, HashSet<PathBuf>>;
}
