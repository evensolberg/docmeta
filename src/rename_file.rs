use crate::utils;
use std::{
    collections::HashMap,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

/// Errors that can occur when renaming a file.
#[derive(Debug, thiserror::Error)]
pub enum RenameError {
    /// No rename pattern was supplied.
    #[error("No rename pattern provided")]
    EmptyPattern,
    /// After tag substitution and sanitisation, the filename stem is empty.
    #[error("No new filename generated")]
    EmptyResult,
    /// The underlying filesystem rename failed.
    #[error("Unable to rename {from} to {to}: {source}")]
    RenameFailed {
        from: String,
        to: String,
        #[source]
        source: std::io::Error,
    },
}

/// Renames the file provided based on the pattern provided.
///
/// **Parameters:**
///
/// - `filename: &str` -- the name of the file to be renamed
/// - `tags: &HashMap<String, String>` -- The metadata values (e.g. Title, Author, Year, Publisher).
/// - `pattern: &str` -- the tag pattern for the new filename. This has been validated to be OK by the CLI.
/// - `dry_run: bool` -- if `true`, log what would happen but do not rename the file.
///
/// Note that you'll need to populate the tags map _before_ using this function. This is to avoid having to re-open the file and re-read the data.
///
/// **Returns**
///
/// - The new file name if successful
/// - A [`RenameError`] variant indicating what failed.
pub fn rename_file(
    filename: &str,
    tags: &HashMap<String, Option<String>>,
    pattern: &str,
    dry_run: bool,
) -> Result<String, RenameError> {
    // Check if there is a rename pattern
    if pattern.is_empty() {
        return Err(RenameError::EmptyPattern);
    }

    let mut new_filename = pattern.to_string();

    // replace any options (eg. %aa, %tg) with the corresponding tag
    new_filename = new_filename.replace("%t", tags.get("Title").and_then(Option::as_deref).unwrap_or("Unknown"));
    new_filename = new_filename.replace("%a", tags.get("Author").and_then(Option::as_deref).unwrap_or("Unknown"));
    new_filename = new_filename.replace("%p", tags.get("Publisher").and_then(Option::as_deref).unwrap_or("Unknown"));
    new_filename = new_filename.replace("%i", tags.get("Identifier").and_then(Option::as_deref).unwrap_or("Unknown"));
    new_filename = new_filename.replace("%y", tags.get("Year").and_then(Option::as_deref).unwrap_or("Unknown"));

    // Semantic replacements: preserve readability where possible.
    new_filename = new_filename.replace('/', "-");
    new_filename = new_filename.replace('\\', "-");
    new_filename = new_filename.replace(':', " -");
    new_filename = new_filename.replace('.', "");

    // Strip characters forbidden on Windows or universally invalid in filenames
    // (e.g. NUL is rejected by every OS).
    new_filename.retain(|ch| !matches!(ch, '*' | '?' | '"' | '<' | '>' | '|' | '\0'));

    // Remove leading or trailing spaces
    new_filename = new_filename.trim().to_string();

    if new_filename.is_empty() {
        return Err(RenameError::EmptyResult);
    }

    // Get the path in front of the filename (eg. "books/book.pdf" returns "books/")
    let parent = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));
    log::debug!("parent = {}", parent.display());

    // Create the full destination path, including the source file's parent directory
    let mut new_path =
        parent.join(Path::new(&new_filename).with_extension(utils::get_extension(filename)));
    log::debug!("new_path = {}", new_path.display());

    // Return if the new filename is the same as the old
    if new_path.to_string_lossy() == filename {
        log::debug!("New filename == old filename. Returning.");
        return Ok(new_path.to_string_lossy().into_owned());
    }

    // Check if a file with the new filename already exists - make the filename unique if it does.
    if new_path.exists() {
        log::warn!("{new_filename} already exists. Appending unique identifier.");
        new_filename = format!("{new_filename} ({:0>4})", get_unique_value());
        new_path =
            parent.join(Path::new(&new_filename).with_extension(utils::get_extension(filename)));
    }

    // Perform the actual rename and check the outcome
    if dry_run {
        log::debug!("dry_run: {filename} --> {}", new_path.display());
    } else {
        // Get parent dir
        let rn_res = std::fs::rename(filename, &new_path);
        match rn_res {
            Ok(()) => log::debug!("{filename} --> {}", new_path.to_string_lossy()),
            Err(source) => {
                return Err(RenameError::RenameFailed {
                    from: filename.to_owned(),
                    to: new_path.to_string_lossy().into_owned(),
                    source,
                });
            }
        }
    }

    // return safely
    let result = new_path.to_string_lossy().into_owned();
    Ok(result)
}

/// Upper bound (exclusive) for the de-collision value: total microseconds relative to
/// `UNIX_EPOCH` modulo this constant, giving a 10-second window of unique values.
const UNIQUE_VALUE_MODULUS: u128 = 10_000_000;

/// Returns a unique numeric identifier in `0..UNIQUE_VALUE_MODULUS` derived from the
/// total microseconds elapsed relative to `UNIX_EPOCH`. Used to de-collide file names.
/// The implementation can be swapped (e.g. for an RNG) without affecting callers.
fn get_unique_value() -> u128 {
    unique_value_from(SystemTime::now())
}

/// Inner implementation so the clock source is injectable in tests.
///
/// For times on or after `UNIX_EPOCH`, uses `time.duration_since(UNIX_EPOCH)`.
/// For times before `UNIX_EPOCH`, falls back to the error's duration (the magnitude
/// of the pre-epoch offset), so the result still varies rather than being a constant.
/// The total microseconds are reduced modulo [`UNIQUE_VALUE_MODULUS`].
fn unique_value_from(time: SystemTime) -> u128 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or_else(|e| e.duration())
        .as_micros()
        % UNIQUE_VALUE_MODULUS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::Duration;
    use tempfile::NamedTempFile;

    fn tags(pairs: &[(&str, &str)]) -> HashMap<String, Option<String>> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), Some((*v).to_string())))
            .collect()
    }

    // ── get_unique_value ────────────────────────────────────────────────────

    #[test]
    fn unique_value_is_within_range() {
        for _ in 0..100 {
            let val = get_unique_value();
            assert!(
                val < UNIQUE_VALUE_MODULUS,
                "expected < {UNIQUE_VALUE_MODULUS}, got {val}"
            );
        }
    }

    #[test]
    fn unique_value_from_before_epoch_is_in_range() {
        // Exercises the Err branch: a time before UNIX_EPOCH causes
        // duration_since to fail; unique_value_from must still return a value
        // in 0..UNIQUE_VALUE_MODULUS rather than panicking.
        // checked_sub guards against platforms that don't support pre-epoch times.
        let Some(before_epoch) = UNIX_EPOCH.checked_sub(Duration::from_secs(1)) else {
            return;
        };
        let val = unique_value_from(before_epoch);
        assert!(
            val < UNIQUE_VALUE_MODULUS,
            "expected < {UNIQUE_VALUE_MODULUS}, got {val}"
        );
    }

    // ── error paths ─────────────────────────────────────────────────────────

    #[test]
    fn empty_pattern_returns_error() {
        let err = rename_file("some_file.epub", &tags(&[]), "", false)
            .expect_err("should fail");
        assert!(matches!(err, RenameError::EmptyPattern));
    }

    #[test]
    fn pattern_that_sanitises_to_empty_returns_error() {
        // Pattern "." becomes "" after the '.' sanitisation step
        let err = rename_file("some_file.epub", &tags(&[]), ".", false)
            .expect_err("should fail");
        assert!(matches!(err, RenameError::EmptyResult));
    }

    // ── tag substitution ────────────────────────────────────────────────────

    #[test]
    fn all_placeholders_are_substituted() {
        let t = tags(&[
            ("Title", "My Book"),
            ("Author", "Jane Doe"),
            ("Publisher", "Acme"),
            ("Identifier", "978-0-00-000000-0"),
            ("Year", "2024"),
        ]);
        // dry_run so no actual file access is needed for the substitution check
        let result = rename_file("placeholder.epub", &t, "%a - %t (%y) [%p] %i", true);
        // The result path ends with the substituted stem + original extension
        let path = result.expect("should succeed");
        assert!(path.contains("Jane Doe"), "author missing: {path}");
        assert!(path.contains("My Book"), "title missing: {path}");
        assert!(path.contains("2024"), "year missing: {path}");
        assert!(path.contains("Acme"), "publisher missing: {path}");
        assert!(
            path.contains("978-0-00-000000-0"),
            "identifier missing: {path}"
        );
    }

    #[test]
    fn missing_tags_fall_back_to_unknown() {
        let result = rename_file("placeholder.epub", &tags(&[]), "%t - %a", true);
        let path = result.expect("should succeed");
        assert!(
            path.contains("Unknown - Unknown"),
            "expected 'Unknown - Unknown' in {path}"
        );
    }

    // ── character sanitisation ───────────────────────────────────────────────

    #[test]
    fn slash_in_tag_is_replaced_with_dash() {
        let t = tags(&[("Title", "A/B")]);
        let result = rename_file("placeholder.epub", &t, "%t", true).expect("ok");
        assert!(result.contains("A-B"), "slash not sanitised: {result}");
    }

    #[test]
    fn colon_in_tag_is_replaced_with_space_dash() {
        let t = tags(&[("Title", "Volume: One")]);
        let result = rename_file("placeholder.epub", &t, "%t", true).expect("ok");
        assert!(
            result.contains("Volume - One"),
            "colon not sanitised: {result}"
        );
    }

    #[test]
    fn dot_in_tag_is_removed() {
        let t = tags(&[("Title", "Mr. Smith")]);
        let result = rename_file("placeholder.epub", &t, "%t", true).expect("ok");
        assert!(result.contains("Mr Smith"), "dot not removed: {result}");
    }

    #[test]
    fn backslash_in_tag_is_replaced_with_dash() {
        let t = tags(&[("Title", r"A\B")]);
        let result = rename_file("placeholder.epub", &t, "%t", true).expect("ok");
        assert!(result.contains("A-B"), "backslash not sanitised: {result}");
    }

    #[test]
    fn windows_forbidden_chars_are_removed() {
        // * ? " < > | are forbidden on Windows
        let t = tags(&[("Title", "A*B?C\"D<E>F|G")]);
        let result = rename_file("placeholder.epub", &t, "%t", true).expect("ok");
        assert!(
            result.contains("ABCDEFG"),
            "forbidden chars not removed: {result}"
        );
    }

    #[test]
    fn pattern_of_only_forbidden_chars_returns_error() {
        // After stripping forbidden chars the stem is empty → error
        let err = rename_file("placeholder.epub", &tags(&[]), "*<>", false)
            .expect_err("should fail");
        assert!(matches!(err, RenameError::EmptyResult));
    }

    #[test]
    fn nul_byte_in_tag_is_removed() {
        let t = tags(&[("Title", "A\0B")]);
        let result = rename_file("placeholder.epub", &t, "%t", true).expect("ok");
        assert!(result.contains("AB"), "NUL byte not removed: {result}");
        assert!(
            !result.contains('\0'),
            "NUL byte present in result: {result}"
        );
    }

    #[test]
    fn rename_to_nonexistent_parent_returns_rename_failed() {
        // Trigger a filesystem error by targeting a directory that does not exist.
        let t = tags(&[("Title", "SomeTitle")]);
        let err = rename_file("nonexistent_dir/source.epub", &t, "%t", false)
            .expect_err("should fail");
        assert!(
            matches!(err, RenameError::RenameFailed { .. }),
            "expected RenameFailed, got: {err}"
        );
    }

    // ── dry run ──────────────────────────────────────────────────────────────

    #[test]
    fn dry_run_does_not_rename_file() {
        let src = NamedTempFile::new().expect("temp file");
        let src_path = src.path().to_string_lossy().to_string();
        let t = tags(&[("Title", "NewName")]);

        let result = rename_file(&src_path, &t, "%t", true).expect("ok");

        // Source still exists
        assert!(
            fs::metadata(&src_path).is_ok(),
            "source was deleted in dry-run"
        );
        // Destination (result path) does not exist
        assert!(
            fs::metadata(&result).is_err(),
            "destination was created in dry-run"
        );
    }

    // ── actual rename ────────────────────────────────────────────────────────

    #[test]
    fn rename_moves_file_to_new_path() {
        let dir = tempfile::tempdir().expect("temp dir");
        let src_path = dir.path().join("source.epub");
        fs::write(&src_path, b"").expect("create src");
        let src_str = src_path.to_string_lossy().to_string();

        let t = tags(&[("Title", "RenamedFile")]);
        let result = rename_file(&src_str, &t, "%t", false).expect("rename should succeed");

        assert!(
            fs::metadata(&src_str).is_err(),
            "source still exists after rename"
        );
        assert!(
            fs::metadata(&result).is_ok(),
            "destination does not exist after rename"
        );
    }

    // ── same-filename guard ──────────────────────────────────────────────────

    #[test]
    fn same_filename_returns_ok_without_moving() {
        let dir = tempfile::tempdir().expect("temp dir");
        let src_path = dir.path().join("mybook.epub");
        fs::write(&src_path, b"").expect("create src");
        let src_str = src_path.to_string_lossy().to_string();

        // Pattern "%t" with Title="mybook" produces "mybook.epub" — same as source
        let t = tags(&[("Title", "mybook")]);
        let result = rename_file(&src_str, &t, "%t", false).expect("ok");

        assert_eq!(result, src_str, "should return the same path unchanged");
        assert!(fs::metadata(&src_str).is_ok(), "file should still exist");
    }
}
