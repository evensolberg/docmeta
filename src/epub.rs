use crate::utils;
use convert_case::{Case, Casing};
use std::{collections::HashMap, error::Error};

use epub::doc::EpubDoc;

/// Get the metadata of an EPUB file and return it as a `HashMap`.
///
/// # Arguments
///
/// * `filename` - A string slice that holds the path to the EPUB file.
///
/// # Returns
///
/// A Result containing a `HashMap` with the metadata of the EPUB file.
/// The `HashMap` contains the following keys:
///
/// * `Title` - The title of the EPUB file.
/// * `Author` - The author of the EPUB file.
/// * `Description` - The description of the EPUB file.
/// * `Publisher` - The publisher of the EPUB file.
/// * `Date` - The date of the EPUB file.
/// * `Language` - The language of the EPUB file.
/// * `Identifier` - The identifier of the EPUB file.
///
/// Keys are title-cased (e.g. `"Title"`, `"Author"`). If a metadata field is
/// absent the key is still present in the map with an empty string value.
///
/// # Example
///
/// ```ignore
/// use std::collections::HashMap;
/// use docmeta::epub::get_metadata;
/// let metadata = get_metadata("tests/test.epub").unwrap();
/// let mut expected_metadata: HashMap<String, String> = HashMap::new();
/// expected_metadata.insert("Title".to_string(), "The Title".to_string());
/// expected_metadata.insert("Author".to_string(), "The Author".to_string());
/// expected_metadata.insert("Description".to_string(), "The Description".to_string());
/// expected_metadata.insert("Publisher".to_string(), "The Publisher".to_string());
/// expected_metadata.insert("Date".to_string(), "2021-01-01".to_string());
/// expected_metadata.insert("Language".to_string(), "en".to_string());
/// expected_metadata.insert("Identifier".to_string(), "urn:isbn:978-3-16-148410-0".to_string());
/// assert_eq!(metadata, expected_metadata);
/// ```
///
/// # Errors
///
/// Returns `Err` if the EPUB file cannot be opened or parsed.
pub fn get_metadata(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let doc = EpubDoc::new(filename)?;
    log::debug!("metadata = {:?}", doc.metadata);

    let mut metadata_map: HashMap<String, String> = HashMap::new();

    let keys = vec![
        "title",
        "author",
        "description",
        "publisher",
        "date",
        "language",
        "identifier",
    ];

    for key in keys {
        if let Some(item) = doc.mdata(key) {
            log::debug!("{key} = {:?}", item.value);
            metadata_map.insert(key.to_string().to_case(Case::Title), item.value.clone());
        } else {
            log::debug!("No {key} found in metadata.");
            metadata_map.insert(key.to_string().to_case(Case::Title), String::new());
        }
    }

    // Extract year from the date string and store it alongside
    let date = metadata_map.get("Date").map_or("", String::as_str).to_owned();
    metadata_map.insert("Year".to_string(), utils::get_year(&date));

    // return the metadata
    log::debug!("metadata_map = {metadata_map:?}");
    Ok(metadata_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_metadata_includes_year_key() {
        let map = get_metadata("tests/fixtures/Mastering.epub").expect("should parse");
        assert!(map.contains_key("Year"), "Year key missing from epub metadata");
    }
}
