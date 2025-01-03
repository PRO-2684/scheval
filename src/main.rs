use scheval::{get_config, run};

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
