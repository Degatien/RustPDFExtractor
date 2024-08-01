use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "rustpdfextractor")]
#[command(version = "0.0.1")]
#[command(about = "A simple pdf parser tool", long_about = None)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the PDF file
    #[arg(short, long, value_name = "FILE")]
    pub path: PathBuf,
}
