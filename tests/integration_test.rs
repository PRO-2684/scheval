use scheval::{run, Config};

const TEST_DIR: &str = "tests/data";

#[test]
fn test_env() {
    // List files in test directory
    let entries = std::fs::read_dir(TEST_DIR).expect("Failed to read directory");
    // Assert that test directory contains a folder named ".vscode"
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
    let config = Config {
        vscode: true,
        suffix: true,
    };
    let result = run(&config, TEST_DIR).expect("Failed to run scheval");
    assert!(result);
}
