use std::ffi::OsStr;
use std::path::Path;

/// Return the file extension of `filename` as a `&str`, or `""` if there is none.
///
/// The extension is returned as-is (original case). Callers that need case-insensitive
/// matching should use [`str::eq_ignore_ascii_case`].
///
/// # Examples
///
/// ```ignore
/// assert_eq!(get_extension("book.epub"), "epub");
/// assert_eq!(get_extension("archive.tar.gz"), "gz");
/// assert_eq!(get_extension("README"), "");
/// assert_eq!(get_extension("BOOK.EPUB"), "EPUB");
/// ```
pub fn get_extension(filename: &str) -> &str {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
}

/// Extract the four-digit year from a date string.
///
/// Handles two formats:
///
/// - A bare four-digit year (`"2024"`) is returned as-is.
/// - A hyphen-separated date (e.g. `"2024-01-01"` or `"2024-03-15T04:00:00+00:00"`) —
///   the part before the first hyphen is returned.
///
/// Any other string is returned unchanged. Returns an empty string only when the input
/// is empty.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(get_year("2024-06-15"), "2024");
/// assert_eq!(get_year("2024-03-15T04:00:00+00:00"), "2024");
/// assert_eq!(get_year("2024"), "2024");
/// assert_eq!(get_year(""), "");
/// ```
pub fn get_year(date: &str) -> String {
    // If it's already a year, just return it
    if date.len() == 4 && date.chars().all(char::is_numeric) {
        return date.to_string();
    }

    let year = if date.contains('-') {
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
    fn test_get_year() {
        assert_eq!(get_year("2020-01-01"), "2020");
        assert_eq!(get_year("2011-03-15T04:00:00+00:00"), "2011");
        assert_eq!(get_year("2020-02-07"), "2020");
        assert_eq!(get_year("2024"), "2024");
    }

    #[test]
    fn test_get_year_edge_cases() {
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

    #[test]
    fn get_extension_preserves_original_case() {
        // get_extension returns the extension as-is; callers use
        // eq_ignore_ascii_case when matching, avoiding a forced allocation.
        assert_eq!(get_extension("BOOK.EPUB"), "EPUB");
        assert_eq!(get_extension("archive.TAR"), "TAR");
    }
}
