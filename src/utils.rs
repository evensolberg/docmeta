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
    log::debug!("date = {}", date);
    let year = if date.starts_with("D:") {
        let subdate = date.split(':').nth(1).unwrap_or("").to_string();
        subdate[0..4].to_string()
    } else {
        date.split('-').next().unwrap_or("").to_string()
    };

    let return_year = year.trim().to_string();
    log::debug!("return_year = {:?}", return_year);

    // return it
    return_year
}

/// Print the metadata
pub fn print_metadata(tags: &std::collections::HashMap<String, String>) {
    if !tags.is_empty() {
        for (key, value) in tags {
            if value.is_empty() {
                println!("{}: N/A", key);
            } else {
                println!("{}: {}", key, value);
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
    use assay::assay;

    #[assay]
    /// Test the get_year function
    fn test_get_year() {
        assert_eq!(get_year("2020-01-01"), "2020");
        assert_eq!(get_year("2011-03-15T04:00:00+00:00"), "2011");
        assert_eq!(get_year("2020-02-07"), "2020");
    }
}
