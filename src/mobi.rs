use mobi::Mobi;

use std::{collections::HashMap, error::Error};

pub fn get_metadata(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mobi_file = Mobi::from_path(filename)?;
    log::debug!("metadata = {:?}", mobi_file.metadata);

    let mut metadata_map: HashMap<String, String> = HashMap::new();

    metadata_map.insert("Title".to_string(), mobi_file.title());
    metadata_map.insert("Author".to_string(), mobi_file.author().unwrap_or_default());
    metadata_map.insert(
        "Description".to_string(),
        mobi_file.description().unwrap_or_default(),
    );
    metadata_map.insert(
        "Publisher".to_string(),
        mobi_file.publisher().unwrap_or_default(),
    );
    metadata_map.insert(
        "Identifier".to_string(),
        mobi_file.isbn().unwrap_or_default(),
    );
    metadata_map.insert(
        "Date".to_string(),
        mobi_file.publish_date().unwrap_or_default(),
    );

    // return the metadata
    Ok(metadata_map)
}
