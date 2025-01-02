//! Test utilities

use std::{env, path::{Path, PathBuf}};

// https://github.com/GrumpyMetalGuy/chwd

/// Data structure for changing the working directory temporarily.
pub struct TempChdir {
    old: PathBuf,
}

impl TempChdir {
    fn change(new: &Path) -> Result<Self, std::io::Error> {
        let old = env::current_dir()?.canonicalize()?;
        env::set_current_dir(&new)?;
        Ok(Self { old })
    }
}

impl Drop for TempChdir {
    fn drop(&mut self) {
        env::set_current_dir(&self.old).expect("Failed to restore the old working directory");
    }
}

/// Change the working directory to `tests/data` temporarily.
pub fn setup() -> TempChdir {
    println!("Current directory: {}", env::current_dir().unwrap().display());
    TempChdir::change(Path::new("tests/data")).expect("Failed to change directory")
}

/// Create a hashset of PathBuf from a list of paths.
pub fn hashset_of_pathbuf(paths: &[&str]) -> std::collections::HashSet<PathBuf> {
    paths.iter().map(|p| PathBuf::from(p)).collect()
}
