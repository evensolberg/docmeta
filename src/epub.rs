use convert_case::{Case, Casing};
use std::{collections::HashMap, error::Error};

use epub::doc::EpubDoc;

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
