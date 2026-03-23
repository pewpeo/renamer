use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::Local;
use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
struct Args {
    #[arg(required = true, num_args = 1..)]
    filenames: Vec<PathBuf>,
}

fn replace_umplauts(filename: &str) -> String {
    filename
        .replace("ä", "ae")
        .replace("ö", "oe")
        .replace("ü", "ue")
        .replace("Ä", "Ae")
        .replace("Ö", "Oe")
        .replace("ß", "ss")
}

fn sanitize_filename(filename: &str) -> String {
    // replace with '_':
    //   - all non-ascii
    //   - the listet ascii characters and spaces (\s)
    //   - more than one '1'
    let re =
        Regex::new(r#"[\x{0080}-\x{FFFF} \.,\"!@#\$%\^&\*\(\)=\+;:<>/\\\|\}\{\[\]`~\s]+|-{2,}"#)
            .unwrap();
    re.replace_all(filename, "_").into_owned()
}

fn prepend_date(filename: &str) -> String {
    let date = Local::now().format("%Y-%m-%d");
    format!("{}_{}", date, filename)
}

fn get_rename_filename(basename: &Path, extname: &str) -> PathBuf {
    let filename = replace_umplauts(&basename.to_string_lossy());
    let filename = sanitize_filename(&filename);
    let filename = prepend_date(&filename);

    format!("{}.{}", filename, extname).into()
}

fn get_rename_filepath(filepath: &Path) -> Option<PathBuf> {
    let dirname = filepath.parent()?;
    let extname = filepath.extension()?;
    let basename = PathBuf::from(filepath.file_stem()?);
    let filename = get_rename_filename(&basename, &extname.to_string_lossy());
    Some(dirname.join(filename))
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
