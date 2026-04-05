use crate::utils;
use convert_case::{Case, Casing};
use std::collections::HashMap;

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
/// * `Date` - The raw date string from the EPUB file.
/// * `Language` - The language of the EPUB file.
/// * `Identifier` - The identifier of the EPUB file.
/// * `Year` - The four-digit year extracted from `Date`.
///
/// Keys are title-cased (e.g. `"Title"`, `"Author"`). Values are `Option<String>`:
/// `None` when the field is absent from the file, `Some(value)` otherwise.
///
/// # Example
///
/// ```ignore
/// use std::collections::HashMap;
/// use docmeta::epub::get_metadata;
/// let metadata = get_metadata("tests/test.epub").unwrap();
/// let mut expected_metadata: HashMap<String, Option<String>> = HashMap::new();
/// expected_metadata.insert("Title".to_string(), Some("The Title".to_string()));
/// expected_metadata.insert("Author".to_string(), Some("The Author".to_string()));
/// expected_metadata.insert("Description".to_string(), Some("The Description".to_string()));
/// expected_metadata.insert("Publisher".to_string(), Some("The Publisher".to_string()));
/// expected_metadata.insert("Date".to_string(), Some("2021-01-01".to_string()));
/// expected_metadata.insert("Language".to_string(), Some("en".to_string()));
/// expected_metadata.insert("Identifier".to_string(), Some("urn:isbn:978-3-16-148410-0".to_string()));
/// assert_eq!(metadata, expected_metadata);
/// ```
///
/// # Errors
///
/// Returns `Err` if the EPUB file cannot be opened or parsed.
pub fn get_metadata(filename: &str) -> anyhow::Result<HashMap<String, Option<String>>> {
    let doc = EpubDoc::new(filename)?;
    log::debug!("metadata = {:?}", doc.metadata);

    let mut metadata_map: HashMap<String, Option<String>> = HashMap::new();

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
        let value = doc.mdata(key).map(|item| {
            log::debug!("{key} = {:?}", item.value);
            item.value.clone()
        });
        if value.is_none() {
            log::debug!("No {key} found in metadata.");
        }
        metadata_map.insert(key.to_string().to_case(Case::Title), value);
    }

    // Extract year from the date string and store it alongside
    let year = metadata_map
        .get("Date")
        .and_then(Option::as_deref)
        .map(utils::get_year)
        .filter(|y| !y.is_empty());
    metadata_map.insert("Year".to_string(), year);

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
        assert_eq!(
            map.get("Year").and_then(Option::as_deref),
            Some("2019"),
            "unexpected Year value"
        );
    }
}
