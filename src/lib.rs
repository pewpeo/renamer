use std::path::{Path, PathBuf};

use chrono::{Local, NaiveDate};
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

fn prepend_date(filename: &str, date: NaiveDate) -> String {
    let formatted = date.format("%Y-%m-%d");
    // preserve existing date prefix
    if filename.starts_with(&formatted.to_string()) {
        return filename.to_string();
    }
    format!("{}_{}", formatted, filename)
}

fn rename_file_str(filename: &str, date: NaiveDate) -> String {
    let filename = replace_umlauts(filename);
    let filename = sanitize_filename(&filename);
    prepend_date(&filename, date)
}

fn get_rename_filename(basename: &Path, extname: &str) -> PathBuf {
    let filename = rename_file_str(&basename.to_string_lossy(), Local::now().date_naive());
    format!("{}.{}", filename, extname).into()
}

pub fn get_rename_filepath(filepath: &Path) -> Option<PathBuf> {
    let dirname = filepath.parent()?;
    let extname = filepath.extension()?;
    let basename = PathBuf::from(filepath.file_stem()?);
    let filename = get_rename_filename(&basename, &extname.to_string_lossy());
    Some(dirname.join(filename))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2023, 4, 25).unwrap()
    }

    #[test]
    fn replaces_umlauts_and_special_chars_with_underscore() {
        assert_eq!(
            rename_file_str("test 123 ü `", date()),
            "2023-04-25_test_123_ue_"
        );
    }

    #[test]
    fn collapses_multiple_spaces() {
        assert_eq!(
            rename_file_str("test    123 ü `", date()),
            "2023-04-25_test_123_ue_"
        );
    }

    #[test]
    fn removes_non_ascii_characters() {
        assert_eq!(rename_file_str("1  è", date()), "2023-04-25_1_");
    }

    #[test]
    fn preserves_single_hyphens() {
        assert_eq!(
            rename_file_str("this-is-a-test", date()),
            "2023-04-25_this-is-a-test"
        );
    }

    #[test]
    fn replaces_consecutive_hyphens_with_underscore() {
        assert_eq!(
            rename_file_str("this-is-a--test", date()),
            "2023-04-25_this-is-a_test"
        );
    }

    #[test]
    fn preserves_existing_date_prefix() {
        assert_eq!(
            rename_file_str("2023-04-25_this_is_a_test", date()),
            "2023-04-25_this_is_a_test"
        );
    }

    
    #[test]
    fn preserves_existing_date_prefix_with_umlauts() {
        assert_eq!(
            rename_file_str("2023-04-25_this_is_ä_test", date()),
            "2023-04-25_this_is_ae_test"
        );
    }
}
