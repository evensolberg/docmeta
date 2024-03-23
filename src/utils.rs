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
pub fn get_year(date: &str) -> String {
    // If it's already a year, just return it
    if date.len() == 4 && date.chars().all(char::is_numeric) {
        return date.to_string();
    }

    let year = if date.starts_with("D:") {
        let subdate = date.split(':').nth(1).unwrap_or("").to_string();
        subdate[0..4].to_string()
    } else if date.contains('-') {
        date.split('-').next().unwrap_or("").to_string()
    } else {
        date.to_string()
    };

    year.trim().to_string()
}

/// Print the metadata
pub fn print_metadata(tags: &std::collections::HashMap<String, String>) {
    if !tags.is_empty() {
        for (key, value) in tags {
            if value.is_empty() {
                println!("{key}: N/A");
            } else {
                println!("{key}: {value}");
            }
        }
    }
}

pub fn new_hashmap() -> std::collections::HashMap<String, String> {
    std::collections::HashMap::new()
}

#[cfg(test)]
///
mod tests {
    use super::*;

    #[test]
    /// Test the get_year function
    fn test_get_year() {
        assert_eq!(get_year("2020-01-01"), "2020");
        assert_eq!(get_year("2011-03-15T04:00:00+00:00"), "2011");
        assert_eq!(get_year("2020-02-07"), "2020");
    }
}
