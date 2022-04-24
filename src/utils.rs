use std::ffi::OsStr;
use std::path::Path;

/// Get the extension part of the filename and return it as a string
pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or_else(|| OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Get the year from a date string
// TODO: Make this more robust
pub fn get_year(date: &str) -> String {
    date.split('-').next().unwrap_or("").to_string()
}

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    /// Test the get_year function
    fn test_get_year() {
        assert_eq!(get_year("2020-01-01"), "2020");
        assert_eq!(get_year("2011-03-15T04:00:00+00:00"), "2011");
        assert_eq!(get_year("2020-02-07"), "2020");
    }
}
