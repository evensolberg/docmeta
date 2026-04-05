use pdf::primitive::PdfString;
use std::{collections::HashMap, error::Error};

/// Convert an optional [`PdfString`] reference to a plain [`String`].
///
/// Returns `"Unknown"` when the option is `None` or the string cannot be decoded
/// as UTF-8. Strips any embedded double-quote characters from the value.
fn pdf_string_to_string(s: Option<&PdfString>) -> String {
    s.and_then(|ps| ps.to_string().ok())
        .unwrap_or_else(|| "Unknown".to_owned())
        .replace('\"', "")
}

/// Extract a named field from a PDF info dictionary into `$mm`.
macro_rules! get_field {
    ($id:ident, $field:ident, $mm:ident, $key:literal) => {
        $mm.insert($key.to_string(), pdf_string_to_string($id.$field.as_ref()));
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
    use pdf::primitive::PdfString;

    #[test]
    fn pdf_string_to_string_returns_unknown_for_none() {
        assert_eq!(pdf_string_to_string(None), "Unknown");
    }

    #[test]
    fn pdf_string_to_string_returns_value_for_some() {
        let ps = PdfString::from("Rust Programming");
        assert_eq!(pdf_string_to_string(Some(&ps)), "Rust Programming");
    }

    #[test]
    fn pdf_string_to_string_strips_double_quotes() {
        let ps = PdfString::from("He said \"hello\"");
        assert_eq!(pdf_string_to_string(Some(&ps)), "He said hello");
    }

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
