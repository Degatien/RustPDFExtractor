#[macro_use]
extern crate clap;
extern crate nom;

use std::{fs::File, io::Read};

use clap::Parser;

mod cmd;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn main() {
    let cli = cmd::Cli::parse();

    println!("path: {:?}", cli.path)
    // if let Some(pdf_content) = pdf_content {
    //     println!("{pdf_content}");
    // }
}
