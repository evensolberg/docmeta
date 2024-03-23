use pdf::primitive::PdfString;
use std::{collections::HashMap, error::Error};

/// Get the metadata from a PDF file and insert it into the provided hashmap.
macro_rules! get_field {
    ($id:ident, $field:ident, $mm:ident, $key:expr) => {
        let mut $field = <Option<PdfString> as Clone>::clone(&$id.$field)
            .unwrap_or(PdfString::from("Unknown"))
            .to_string()
            .unwrap_or(String::from("Unknown"));
        $field = $field.replace('\"', "");
        $mm.insert($key.to_string(), $field);
    };
}

/// get the metadata from a PDF file
pub fn get_metadata(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    log::debug!("Opening file: {filename}");

    let file = pdf::file::FileOptions::cached().open(filename)?;
    let Some(info) = file.trailer.info_dict.as_ref() else {
        return Err("No info dictionary found in {filename}".into());
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
        metadata_map.insert("Year".to_string(), "Unknown".to_string());
    }

    log::debug!("metadata_map: {metadata_map:?}");

    Ok(metadata_map)
}
