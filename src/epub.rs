use convert_case::{Case, Casing};
use std::{collections::HashMap, error::Error};

use epub::doc::EpubDoc;

macro_rules! insert_metadata {
    ($metadata_map:expr, $metadata:expr, $key:expr) => {
        if let Some(value) = $metadata.get($key) {
            log::debug!("{} = {value:?}", $key);
            $metadata_map.insert(
                $key.to_string().to_case(Case::Title),
                value
                    .first()
                    .unwrap_or(&String::from("Unknown"))
                    .to_string(),
            );
        } else {
            log::debug!("No {} found in metadata.", $key);
            $metadata_map.insert(
                $key.to_string().to_case(Case::Title),
                String::from("Unknown"),
            );
        }
    };
}

pub fn get_metadata(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let doc = EpubDoc::new(filename)?;
    let metadata = doc.metadata;
    log::debug!("metadata = {metadata:?}");

    let mut metadata_map: HashMap<String, String> = HashMap::new();

    insert_metadata!(metadata_map, metadata, "title");
    insert_metadata!(metadata_map, metadata, "author");
    insert_metadata!(metadata_map, metadata, "description");
    insert_metadata!(metadata_map, metadata, "publisher");
    insert_metadata!(metadata_map, metadata, "date");
    insert_metadata!(metadata_map, metadata, "language");
    insert_metadata!(metadata_map, metadata, "identifier");

    // return the metadata
    log::debug!("metadata_map = {metadata_map:?}");
    Ok(metadata_map)
}
