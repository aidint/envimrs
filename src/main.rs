use clap::Parser;
use envim;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clargs = envim::cli::ClArgs::parse();
    envim::run(&clargs)
}
