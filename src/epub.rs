use convert_case::{Case, Casing};
use std::{collections::HashMap, error::Error};

use epub::doc::EpubDoc;

/// Get the metadata of an EPUB file and return it as a HashMap.
///
/// # Arguments
///
/// * `filename` - A string slice that holds the path to the EPUB file.
///
/// # Returns
///
/// A Result containing a HashMap with the metadata of the EPUB file.
/// The HashMap contains the following keys:
///
/// * `title` - The title of the EPUB file.
/// * `author` - The author of the EPUB file.
/// * `description` - The description of the EPUB file.
/// * `publisher` - The publisher of the EPUB file.
/// * `date` - The date of the EPUB file.
/// * `language` - The language of the EPUB file.
/// * `identifier` - The identifier of the EPUB file.
///
/// If the metadata is not found, the value will be set to "Unknown".
///
/// # Example
///
/// ```ignore
/// use std::collections::HashMap;
/// use epub_metadata::get_metadata;
/// let metadata = get_metadata("tests/test.epub").unwrap();
/// let mut expected_metadata: HashMap<String, String> = HashMap::new();
/// expected_metadata.insert("title".to_string(), "The Title".to_string());
/// expected_metadata.insert("author".to_string(), "The Author".to_string());
/// expected_metadata.insert("description".to_string(), "The Description".to_string());
/// expected_metadata.insert("publisher".to_string(), "The Publisher".to_string());
/// expected_metadata.insert("date".to_string(), "2021-01-01".to_string());
/// expected_metadata.insert("language".to_string(), "en".to_string());
/// expected_metadata.insert("identifier".to_string(), "urn:isbn:978-3-16-148410-0".to_string());
/// assert_eq!(metadata, expected_metadata);
/// ```
///
/// # Errors
///
/// If the EPUB file is not found or if there is an error reading the EPUB file,
/// a Box<dyn Error> will be returned.
/// If the metadata is not found, a Box<dyn Error> will be returned.
/// If the metadata is not found, the value will be set to "Unknown".

pub fn get_metadata(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let doc = EpubDoc::new(filename)?;
    let metadata = doc.metadata;
    log::debug!("metadata = {metadata:?}");

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
        if let Some(value) = metadata.get(key) {
            log::debug!("{key} = {value:?}");
            metadata_map.insert(
                key.to_string().to_case(Case::Title),
                value
                    .first()
                    .unwrap_or(&String::from("Unknown"))
                    .to_string(),
            );
        } else {
            log::debug!("No {key} found in metadata.");
            metadata_map.insert(
                key.to_string().to_case(Case::Title),
                String::from("Unknown"),
            );
        }
    }

    // return the metadata
    log::debug!("metadata_map = {metadata_map:?}");
    Ok(metadata_map)
}
