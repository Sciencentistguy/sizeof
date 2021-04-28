use std::{self, io, path::PathBuf};

use structopt::StructOpt;

fn main() -> Result<(), io::Error> {
    let opt = Opt::from_args();
    if !opt.path.is_file() {
        if opt.path.is_dir() {
            eprintln!("sizeof only works on files");
        } else {
            eprintln!(
                "sizeof: error file not found: {}",
                opt.path
                    .to_str()
                    .unwrap_or("<path contains invalid unicode so cannot be displayed>")
            );
        }
        return Ok(());
    }
    let file = std::fs::File::open(opt.path)?;
    let meta = file.metadata()?;
    println!("{}", meta.len());
    Ok(())
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt()]
    /// The path to the file.
    path: PathBuf,
}
