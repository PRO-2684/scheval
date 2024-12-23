use clap::Parser;
use scheval::{Args, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run(&args)
}
