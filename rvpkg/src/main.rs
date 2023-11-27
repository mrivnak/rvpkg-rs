use clap::Parser;

/// Source-based package manager for Linux
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
}

fn main() {
    let args = Args::parse();
}