use scheval::{get_config, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = get_config();
    match run(&args, ".") {
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
