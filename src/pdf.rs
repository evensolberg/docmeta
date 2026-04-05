use pdf::primitive::PdfString;
use std::collections::HashMap;

/// Errors that can occur when reading PDF metadata.
#[derive(Debug, thiserror::Error)]
pub enum PdfMetaError {
    /// The PDF file could not be opened or parsed.
    #[error(transparent)]
    Pdf(#[from] pdf::PdfError),
    /// The PDF contains no info dictionary.
    #[error("No info dictionary found in {0}")]
    NoInfoDict(String),
}

/// Convert an optional [`PdfString`] reference to an `Option<String>`.
///
/// Returns `None` when the option is `None` or the string cannot be decoded as
/// UTF-8. Strips any embedded double-quote characters from the value.
fn pdf_string_to_string(s: Option<&PdfString>) -> Option<String> {
    s.and_then(|ps| ps.to_string().ok())
        .map(|v| v.replace('\"', ""))
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
/// - The PDF contains no info dictionary — returns [`PdfMetaError::NoInfoDict`] carrying `filename`.
pub fn get_metadata(filename: &str) -> Result<HashMap<String, Option<String>>, PdfMetaError> {
    log::debug!("Opening file: {filename}");

    let file = pdf::file::FileOptions::cached().open(filename)?;
    let Some(info) = file.trailer.info_dict.as_ref() else {
        return Err(PdfMetaError::NoInfoDict(filename.to_owned()));
    };

    let mut metadata_map: HashMap<String, Option<String>> = HashMap::new();

    get_field!(info, author, metadata_map, "Author");
    get_field!(info, title, metadata_map, "Title");
    get_field!(info, subject, metadata_map, "Subject");
    get_field!(info, keywords, metadata_map, "Keywords");
    get_field!(info, creator, metadata_map, "Creator");
    get_field!(info, producer, metadata_map, "Producer");

    metadata_map.insert(
        "Year".to_string(),
        info.creation_date
            .as_ref()
            .map(|d| d.year.to_string()),
    );

    log::debug!("metadata_map: {metadata_map:?}");

    Ok(metadata_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pdf::primitive::PdfString;

    #[test]
    fn pdf_string_to_string_returns_none_for_none() {
        assert_eq!(pdf_string_to_string(None), None);
    }

    #[test]
    fn pdf_string_to_string_returns_some_for_present_value() {
        let ps = PdfString::from("Rust Programming");
        assert_eq!(pdf_string_to_string(Some(&ps)), Some("Rust Programming".to_owned()));
    }

    #[test]
    fn pdf_string_to_string_strips_double_quotes() {
        let ps = PdfString::from("He said \"hello\"");
        assert_eq!(pdf_string_to_string(Some(&ps)), Some("He said hello".to_owned()));
    }

    #[test]
    fn error_is_no_info_dict_variant() {
        let filename = "tests/fixtures/no-info-dict.pdf";
        let err = get_metadata(filename).expect_err("expected error for PDF with no info dict");
        assert!(
            matches!(err, PdfMetaError::NoInfoDict(ref f) if f == filename),
            "expected NoInfoDict(\"{filename}\"), got: {err}"
        );
    }
}
