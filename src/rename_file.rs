use crate::utils;
use std::{
    collections::HashMap,
    error::Error,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

/// Renames the file provided based on the pattern provided.
///
/// **Parameters:**
///
/// - `filename: &str` -- the name of the file to be renamed
/// - `tags: &HashMap<String, String>` -- The various tag values (ie. Album Artist, Genre, etc.)
/// - `pattern: &str` -- the tag pattern for the new filename. This has been validated to be OK by the CLI.
/// - `config: &DefaultValues` -- The tags that have been set, and any config settings such as dry-run.
///
/// Note that you'll need to populate the tags struct _before_ using this function. This is to avoid having to re-open the file and re-read the data.
///
/// **Returns**
///
/// - The new file name if successful
/// - An error message if unsuccessful.
pub fn rename_file(
    filename: &str,
    tags: &HashMap<String, String>,
    pattern: &str,
    dry_run: bool,
) -> Result<String, Box<dyn Error>> {
    // Check if there is a rename pattern
    if pattern.is_empty() {
        return Err("No rename pattern provided".into());
    }

    let mut new_filename = pattern.to_string();

    // replace any options (eg. %aa, %tg) with the corresponding tag
    new_filename = new_filename.replace("%t", tags.get("Title").unwrap_or(&"Unknown".to_string()));
    new_filename = new_filename.replace("%a", tags.get("Author").unwrap_or(&"Unknown".to_string()));
    new_filename = new_filename.replace(
        "%p",
        tags.get("Publisher").unwrap_or(&"Unknown".to_string()),
    );
    new_filename = new_filename.replace(
        "%i",
        tags.get("Identifier").unwrap_or(&"Unknown".to_string()),
    );
    new_filename = new_filename.replace("%y", tags.get("Year").unwrap_or(&"Unknown".to_string()));

    // Fix a few things we know will give us trouble later.
    new_filename = new_filename.replace('/', "-");
    new_filename = new_filename.replace(':', " -");
    new_filename = new_filename.replace('.', "");

    // Remove leading or trailing spaces
    new_filename = new_filename.trim().to_string();

    if new_filename.is_empty() {
        return Err("No new filename generated".into());
    }

    // Create the new filename
    let mut new_path = Path::new(&new_filename).with_extension(utils::get_extension(filename));
    log::debug!("new_path = {new_path:?}");

    // Return if the new filename is the same as the old
    let np = new_path.to_string_lossy().to_string();
    if np == *filename {
        log::debug!("New filename == old filename. Returning.");
        return Ok(np);
    }

    // Get the path in front of the filename (eg. "books/book.pdf" returns "books/")
    let parent = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));
    log::debug!("parent = {parent:?}");

    // Check if a file with the new filename already exists - make the filename unique if it does.
    if Path::new(&new_path).exists() {
        log::warn!("{new_filename} already exists. Appending unique identifier.");
        new_filename = format!("{new_filename} ({:0>4})", get_unique_value());
    }

    new_path = parent.join(Path::new(&new_filename).with_extension(utils::get_extension(filename)));

    // Perform the actual rename and check the outcome
    if dry_run {
        log::debug!("dry_run: {filename} --> {}", new_path.display());
    } else {
        // Get parent dir
        let rn_res = std::fs::rename(filename, &new_path);
        match rn_res {
            Ok(()) => log::debug!("{filename} --> {}", new_path.to_string_lossy()),
            Err(err) => {
                return Err(format!(
                    "Unable to rename {filename} to {}. Error message: {err}",
                    new_path.to_string_lossy(),
                )
                .into())
            }
        }
    }

    // return safely
    let result = new_path.to_string_lossy().into_owned();
    Ok(result)
}

/// Gets the microsecond part of the current duration since `UNIX_EPOCH` and modulate to a 4-digit number.
/// This is used to ensure uniqueness of file names.
/// This can be changed to something else later without impacting the main application.
/// For example, one could switch to a random number generator or something.
fn get_unique_value() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards. You probably have bigger things to worry about.")
        .as_micros()
        % 10_000_000
}
