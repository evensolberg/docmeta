use std::{collections::HashMap, error::Error};

use epub::doc::EpubDoc;

fn get_metadata(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let doc = EpubDoc::new(filename)?;
    let metadata = doc.metadata;
    log::debug!("metadata = {:?}", metadata);

    let mut metadata_map: HashMap<String, String> = HashMap::new();
    if let Some(title) = metadata.get("title") {
        log::debug!("title = {:?}", title);
        metadata_map.insert(
            "Title".to_string(),
            title.first().unwrap_or(&"".to_string()).to_string(),
        );
    } else {
        log::debug!("No title found in metadata.");
        metadata_map.insert("Title".to_string(), "".to_string());
    }

    if let Some(author) = metadata.get("author") {
        log::debug!("author = {:?}", author);
        metadata_map.insert(
            "Author".to_string(),
            author.first().unwrap_or(&"".to_string()).to_string(),
        );
    } else {
        log::debug!("No author found in metadata.");
        metadata_map.insert("Author".to_string(), "".to_string());
    }

    if let Some(description) = metadata.get("description") {
        log::debug!("description = {:?}", description);
        metadata_map.insert(
            "Description".to_string(),
            description.first().unwrap_or(&"".to_string()).to_string(),
        );
    } else {
        log::debug!("No description found in metadata.");
        metadata_map.insert("Description".to_string(), "".to_string());
    }

    if let Some(publisher) = metadata.get("publisher") {
        log::debug!("publisher = {:?}", publisher);
        metadata_map.insert(
            "Publisher".to_string(),
            publisher.first().unwrap_or(&"".to_string()).to_string(),
        );
    } else {
        log::debug!("No publisher found in metadata.");
        metadata_map.insert("Publisher".to_string(), "".to_string());
    }

    if let Some(date) = metadata.get("date") {
        log::debug!("date = {:?}", date);
        metadata_map.insert(
            "Date".to_string(),
            date.first().unwrap_or(&"".to_string()).to_string(),
        );
    } else {
        log::debug!("No date found in metadata.");
        metadata_map.insert("Date".to_string(), "".to_string());
    }

    if let Some(language) = metadata.get("language") {
        log::debug!("language = {:?}", language);
        metadata_map.insert(
            "Language".to_string(),
            language.first().unwrap_or(&"".to_string()).to_string(),
        );
    } else {
        log::debug!("No language found in metadata.");
        metadata_map.insert("Language".to_string(), "".to_string());
    }

    if let Some(identifier) = metadata.get("identifier") {
        log::debug!("identifier = {:?}", identifier);
        metadata_map.insert(
            "Identifier".to_string(),
            identifier.first().unwrap_or(&"".to_string()).to_string(),
        );
    } else {
        log::debug!("No identifier found in metadata.");
        metadata_map.insert("Identifier".to_string(), "".to_string());
    }

    // return the metadata
    log::debug!("metadata_map = {:?}", metadata_map);
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
