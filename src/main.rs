// src/main.rs
/*
 * Main executable for FluxLoom
 */

use clap::Parser;
use fluxloom::{Result, run};

#[derive(Parser)]
#[command(version, about = "FluxLoom - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
