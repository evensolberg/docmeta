use crate::utils;
use mobi::Mobi;
use std::collections::HashMap;

/// Read metadata from a MOBI file and return it as a [`HashMap`].
///
/// # Arguments
///
/// * `filename` - Path to the MOBI file to read.
///
/// # Returns
///
/// A [`HashMap`] containing the following keys. Values are `Option<String>`:
/// `None` when the field is absent, `Some(value)` otherwise.
///
/// | Key | Source field |
/// |-----|-------------|
/// | `"Title"` | Book title |
/// | `"Author"` | Author name, or `None` if absent |
/// | `"Description"` | Book description, or `None` if absent |
/// | `"Publisher"` | Publisher name, or `None` if absent |
/// | `"Identifier"` | ISBN, or `None` if absent |
/// | `"Date"` | Publish date string, or `None` if absent |
/// | `"Year"` | Four-digit year extracted from `Date`, or `None` if absent |
///
/// # Errors
///
/// Returns `Err` if the file cannot be opened or parsed as a MOBI document.
pub fn get_metadata(filename: &str) -> anyhow::Result<HashMap<String, Option<String>>> {
    let mobi_file = Mobi::from_path(filename)?;
    log::debug!("metadata = {:?}", mobi_file.metadata);

    let mut metadata_map: HashMap<String, Option<String>> = HashMap::new();

    metadata_map.insert("Title".to_string(), Some(mobi_file.title()));
    metadata_map.insert("Author".to_string(), mobi_file.author());
    metadata_map.insert("Description".to_string(), mobi_file.description());
    metadata_map.insert("Publisher".to_string(), mobi_file.publisher());
    metadata_map.insert("Identifier".to_string(), mobi_file.isbn());
    metadata_map.insert("Date".to_string(), mobi_file.publish_date());

    // Extract year from the date string and store it alongside
    let year = metadata_map
        .get("Date")
        .and_then(Option::as_deref)
        .map(utils::get_year)
        .filter(|y| !y.is_empty());
    metadata_map.insert("Year".to_string(), year);

    log::debug!("metadata_map = {metadata_map:?}");

    // return the metadata
    Ok(metadata_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_metadata_includes_year_key() {
        let map = get_metadata("tests/fixtures/Mastering.mobi").expect("should parse");
        assert_eq!(
            map.get("Year").and_then(Option::as_deref),
            Some("2019"),
            "unexpected Year value"
        );
    }
}
