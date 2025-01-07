//! Main entry point for the scheval command line tool

use scheval::{get_config, run};

/// Parse arguments and run scheval
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = get_config();
    match run(&cfg, ".") {
        Ok(success) => {
            if success {
                Ok(())
            } else {
                Err("Validation failed".into())
            }
        }
        Err(err) => Err(err),
    }
}
