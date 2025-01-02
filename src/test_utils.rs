//! Test utilities

use std::path::PathBuf;

/// Public constant for the path to the test data directory.
pub const TEST_DIR: &str = "tests/data";

/// Create a hashset of PathBuf from a list of paths.
pub fn hashset_of_pathbuf(paths: &[&str]) -> std::collections::HashSet<PathBuf> {
    paths.iter().map(|p| PathBuf::from(p)).collect()
}
