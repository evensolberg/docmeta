use pdf::primitive::PdfString;
use std::{collections::HashMap, error::Error};

/// Extract a named field from a PDF info dictionary into `$mm`.
///
/// Falls back to `"Unknown"` when the field is absent or cannot be converted to a
/// UTF-8 string, and strips any embedded double-quote characters from the value.
macro_rules! get_field {
    ($id:ident, $field:ident, $mm:ident, $key:literal) => {
        let mut $field = <Option<PdfString> as Clone>::clone(&$id.$field)
            .unwrap_or(PdfString::from("Unknown"))
            .to_string()
            .unwrap_or(String::from("Unknown"));
        $field = $field.replace('\"', "");
        $mm.insert($key.to_string(), $field);
    };
}

/// Read metadata from a PDF file and return it as a [`HashMap`].
///
/// # Arguments
///
/// * `filename` - Path to the PDF file to read.
///
/// # Returns
///
/// A [`HashMap`] containing the following keys (all `String` values):
///
/// | Key | Source field |
/// |-----|-------------|
/// | `"Author"` | `info.author` |
/// | `"Title"` | `info.title` |
/// | `"Subject"` | `info.subject` |
/// | `"Keywords"` | `info.keywords` |
/// | `"Creator"` | `info.creator` |
/// | `"Producer"` | `info.producer` |
/// | `"Year"` | `info.creation_date.year` |
///
/// Fields absent from the PDF info dictionary are inserted with the value `"Unknown"`,
/// except `"Year"` which is inserted as an empty string when the creation date is missing.
///
/// # Errors
///
/// Returns `Err` in two distinct cases:
/// - The `pdf` crate cannot open or parse the file (corrupt data, unsupported version,
///   permission denied, etc.) — the underlying crate error is propagated.
/// - The PDF contains no info dictionary — the error message includes `filename`.
pub fn get_metadata(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    log::debug!("Opening file: {filename}");

    let file = pdf::file::FileOptions::cached().open(filename)?;
    let Some(info) = file.trailer.info_dict.as_ref() else {
        return Err(format!("No info dictionary found in {filename}").into());
    };

    let mut metadata_map: HashMap<String, String> = HashMap::new();

    get_field!(info, author, metadata_map, "Author");
    get_field!(info, title, metadata_map, "Title");
    get_field!(info, subject, metadata_map, "Subject");
    get_field!(info, keywords, metadata_map, "Keywords");
    get_field!(info, creator, metadata_map, "Creator");
    get_field!(info, producer, metadata_map, "Producer");

    if let Some(creation_date) = &info.creation_date {
        metadata_map.insert("Year".to_string(), creation_date.year.to_string());
    } else {
        metadata_map.insert("Year".to_string(), String::new());
    }

    log::debug!("metadata_map: {metadata_map:?}");

    Ok(metadata_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_message_contains_filename_when_no_info_dict() {
        let filename = "tests/fixtures/no-info-dict.pdf";
        let result = get_metadata(filename);
        assert!(
            result.is_err(),
            "Expected an error for a PDF with no info dict"
        );
        let msg = result.expect_err("should fail").to_string();
        assert!(
            msg.starts_with("No info dictionary found in "),
            "Error message should start with 'No info dictionary found in ', got: {msg}"
        );
        assert!(
            msg.contains(filename),
            "Error message should contain the filename '{filename}', got: {msg}"
        );
    }
}
