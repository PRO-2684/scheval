use std::{env, path::{Path, PathBuf}};
use scheval::{run, Config};

// https://github.com/GrumpyMetalGuy/chwd
struct TempChdir {
    old: PathBuf,
}

impl TempChdir {
    fn change(new: &Path) -> Result<Self, std::io::Error> {
        let old = env::current_dir()?;
        env::set_current_dir(&new)?;
        Ok(Self { old })
    }
}

impl Drop for TempChdir {
    fn drop(&mut self) {
        env::set_current_dir(&self.old).expect("Failed to restore the old working directory");
    }
}

fn setup() -> TempChdir {
    TempChdir::change(Path::new("tests/data")).expect("Failed to change directory")
}

#[test]
fn test_env() {
    let _dir_change = setup();
    println!("Current directory: {}", env::current_dir().unwrap().display());
    // List the files in the current directory
    let entries = std::fs::read_dir(".").expect("Failed to read directory");
    // Assert that the current directory contains a folder named exactly ".vscode"
    let mut vscode_found = false;
    for entry in entries {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();
        println!("{}", path.display());
        if path.is_dir() && path.file_name().unwrap() == ".vscode" {
            vscode_found = true;
            break;
        }
    }
    assert!(vscode_found);
}

#[test]
fn test_scheval() {
    let _dir_change = setup();
    let config = Config {
        vscode: true,
        suffix: true,
    };
    let result = run(&config).expect("Failed to run scheval");
    assert!(result);
}
