use std::{collections::HashMap, error::Error};

// use pdf::error::PdfError;
use pdf::file::File;
// use pdf::object::{FieldDictionary, FieldType, Resolve};

/// get the metadata from a PDF file
fn get_metadata(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    log::debug!("Opening file: {}", filename);
    let file = match File::<Vec<u8>>::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("Error opening file: {}. Error: {}", filename, e).into());
        }
    };

    let metadata = file.trailer.info_dict.unwrap();
    log::debug!("metadata = {:?}", metadata);

    let mut metadata_map: HashMap<String, String> = HashMap::new();
    for (key, value) in &metadata {
        log::debug!("{}: {:?}", key, value);
        let key_str = key.to_string();
        let value_str = value.to_string().replace('\"', "");
        metadata_map.insert(key_str, value_str);
    }

    if let Some(title) = metadata.get("Title") {
        log::debug!("title = {:?}", title);
        metadata_map.insert("Title".to_string(), title.to_string().replace('\"', ""));
    } else {
        log::debug!("No title found in metadata.");
        metadata_map.insert("Title".to_string(), "".to_string());
    }

    if let Some(author) = metadata.get("Author") {
        log::debug!("author = {:?}", author);
        metadata_map.insert("Author".to_string(), author.to_string().replace('\"', ""));
    } else {
        log::debug!("No author found in metadata.");
        metadata_map.insert("Author".to_string(), "".to_string());
    }

    if let Some(publisher) = metadata.get("EBX_PUBLISHER") {
        log::debug!("publisher = {:?}", publisher);
        metadata_map.insert(
            "Publisher".to_string(),
            publisher
                .to_string()
                .replace('\"', "")
                .replace('/', "")
                .replace("#20", " "),
        );
    } else {
        log::debug!("No publisher found in metadata.");
        metadata_map.insert("Publisher".to_string(), "".to_string());
    }

    log::debug!("metadata_map = {:?}", metadata_map);
    log::debug!("metadata_map.len() = {}", metadata_map.len());
    for (key, value) in &metadata_map {
        log::debug!("{}: {}", key, value);
    }

    // return safely
    Ok(metadata_map)
}

/// Print the ePub metadata
pub fn print_metadata(filename: &str) -> Result<(), Box<dyn Error>> {
    let metadata = get_metadata(filename)?;
    for (key, mut value) in metadata {
        if value.is_empty() {
            value = "N/A".to_string();
        }
        println!("{}: {}", key, value);
    }

    // return safely
    Ok(())
}