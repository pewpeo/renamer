use std::path::{Path, PathBuf};

use chrono::Local;
use regex::Regex;

fn replace_umlauts(filename: &str) -> String {
    filename
        .replace("ä", "ae")
        .replace("ö", "oe")
        .replace("ü", "ue")
        .replace("Ä", "Ae")
        .replace("Ö", "Oe")
        .replace("ß", "ss")
}

fn sanitize_filename(filename: &str) -> String {
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
    let filename = replace_umlauts(&basename.to_string_lossy());
    let filename = sanitize_filename(&filename);
    let filename = prepend_date(&filename);

    format!("{}.{}", filename, extname).into()
}

pub fn get_rename_filepath(filepath: &Path) -> Option<PathBuf> {
    let dirname = filepath.parent()?;
    let extname = filepath.extension()?;
    let basename = PathBuf::from(filepath.file_stem()?);
    let filename = get_rename_filename(&basename, &extname.to_string_lossy());
    Some(dirname.join(filename))
}
