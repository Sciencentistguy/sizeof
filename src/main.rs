use std::{self, io, path::PathBuf};

use clap::Parser;

fn main() -> Result<(), io::Error> {
    let opt = Opt::parse();
    if !opt.path.is_file() {
        if opt.path.is_dir() {
            eprintln!("sizeof: sizeof only works on files");
        } else {
            eprintln!(
                "sizeof: error file not found: {}",
                opt.path.file_name().unwrap().to_string_lossy()
            );
        }
        return Ok(());
    }
    let file = std::fs::File::open(opt.path)?;
    let meta = file.metadata()?;
    println!("{}", meta.len());
    Ok(())
}

#[derive(Parser, Debug)]
struct Opt {
    /// The path to the file.
    path: PathBuf,
}
