use scheval::{run, Config, test_utils::setup};

#[test]
fn test_env() {
    let _dir_change = setup();
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
