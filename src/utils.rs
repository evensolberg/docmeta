use std::ffi::OsStr;
use std::path::Path;

/// Return the lowercase file extension of `filename`, or an empty string if there is none.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(get_extension("book.epub"), "epub");
/// assert_eq!(get_extension("archive.tar.gz"), "gz");
/// assert_eq!(get_extension("README"), "");
/// ```
pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or_else(|| OsStr::new(""))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Extract the four-digit year from a date string.
///
/// Handles three common formats:
///
/// - A bare four-digit year (`"2024"`) is returned as-is.
/// - A PDF date string prefixed with `"D:"` (e.g. `"D:20240101120000+00'00'"`) — the four
///   digits immediately after the prefix are returned.
/// - An ISO 8601 / hyphen-separated date (e.g. `"2024-01-01"`) — the part before the first
///   hyphen is returned.
///
/// Returns an empty string when the year cannot be extracted from a recognised format.
/// Any string that does not match the above patterns is returned unchanged.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(get_year("2024-06-15"), "2024");
/// assert_eq!(get_year("D:20240615120000+00'00'"), "2024");
/// assert_eq!(get_year("2024"), "2024");
/// assert_eq!(get_year("D:"), "");
/// ```
pub fn get_year(date: &str) -> String {
    // If it's already a year, just return it
    if date.len() == 4 && date.chars().all(char::is_numeric) {
        return date.to_string();
    }

    let year = if date.starts_with("D:") {
        let subdate = date.strip_prefix("D:").unwrap_or("");
        subdate.get(0..4).unwrap_or("").to_string()
    } else if date.contains('-') {
        date.split('-').next().unwrap_or("").to_string()
    } else {
        date.to_string()
    };

    year.trim().to_string()
}

/// Print each metadata key/value pair to stdout.
///
/// Empty values are displayed as `"N/A"` rather than a blank.
pub fn print_metadata(tags: &std::collections::HashMap<String, String>) {
    for (key, value) in tags {
        if value.is_empty() {
            println!("{key}: N/A");
        } else {
            println!("{key}: {value}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test the `get_year` function
    fn test_get_year() {
        assert_eq!(get_year("2020-01-01"), "2020");
        assert_eq!(get_year("2011-03-15T04:00:00+00:00"), "2011");
        assert_eq!(get_year("2020-02-07"), "2020");
        assert_eq!(get_year("D:20200207123456+00'00'"), "2020");
        assert_eq!(get_year("D:20230101000000+05:30"), "2023");
        assert_eq!(get_year("2024"), "2024");
    }

    #[test]
    fn test_get_year_edge_cases() {
        assert_eq!(get_year("D:"), "");
        assert_eq!(get_year("D:202"), "");
        assert_eq!(get_year(""), "");
        assert_eq!(get_year("unknown"), "unknown");
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(get_extension("file.txt"), "txt");
        assert_eq!(get_extension("image.jpg"), "jpg");
        assert_eq!(get_extension("document.pdf"), "pdf");
        assert_eq!(get_extension("document.xyz.pdf"), "pdf");
        assert_eq!(get_extension("no_extension"), "");
    }
}
