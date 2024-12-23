use scheval::{get_config, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = get_config();
    run(&args)
}
