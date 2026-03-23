use std::{fs, path::PathBuf};

use clap::Parser;
use renamer_rust::get_rename_filepath;

#[derive(Debug, Parser)]
struct Args {
    #[arg(required = true, num_args = 1..)]
    filenames: Vec<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    for filename in args.filenames {
        let filepath = match get_rename_filepath(&filename) {
            Some(filepath) => filepath,
            None => {
                return Err(std::io::Error::other("Failed to get rename filepath"));
            }
        };
        println!("Rename {:?} to {:?}", filename, filepath.to_string_lossy());
        fs::rename(filename, filepath)?;
    }
    Ok(())
}
