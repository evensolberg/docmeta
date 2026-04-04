use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &["epub", "mobi", "pdf"];

/// Collect file paths from the supplied inputs.
///
/// Each entry in `inputs` is treated as follows:
///
/// - **File**: included as-is (regardless of extension or `recursive`).
/// - **Directory** with `recursive = true`: walked depth-first; only files
///   whose extensions appear in [`SUPPORTED_EXTENSIONS`] are included.
/// - **Directory** with `recursive = false`: skipped with a warning.
/// - Anything that cannot be stat'd (does not exist, permission denied, etc.): skipped with a warning.
///
/// The returned list follows the order of `inputs`: each input's contribution
/// (the path itself for files, or the sorted directory contents for directories)
/// is appended when that input is encountered.
pub fn collect_files(inputs: &[String], recursive: bool) -> Vec<String> {
    let mut result = Vec::new();

    for input in inputs {
        let meta = match std::fs::metadata(input) {
            Ok(meta) => meta,
            Err(err) => {
                log::warn!("Failed to stat path, skipping: {input} ({err})");
                continue;
            }
        };

        if meta.is_file() {
            result.push(input.clone());
        } else if meta.is_dir() {
            if !recursive {
                log::warn!("Directory skipped (use --recursive to traverse): {input}");
                continue;
            }
            for entry in WalkDir::new(input)
                .follow_links(true)
                .sort_by_file_name()
                .into_iter()
                .filter_map(|e| match e {
                    Ok(entry) => Some(entry),
                    Err(err) => {
                        match (err.path(), err.io_error()) {
                            (Some(path), Some(io_err)) => {
                                log::warn!("Skipping {} ({io_err})", path.display());
                            }
                            _ => log::warn!("Skipping entry: {err}"),
                        }
                        None
                    }
                })
                .filter(|e| e.file_type().is_file())
            {
                let path = entry.path();
                let ext_matches = path.extension().is_some_and(|ext| {
                    SUPPORTED_EXTENSIONS
                        .iter()
                        .any(|s| ext.eq_ignore_ascii_case(s))
                });
                if ext_matches {
                    match path.to_str() {
                        Some(s) => result.push(s.to_owned()),
                        None => log::warn!("Skipping non-UTF-8 path: {}", path.display()),
                    }
                }
            }
        } else {
            log::warn!("Skipping unsupported file type (not a file or directory): {input}");
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // ── non-existent path ────────────────────────────────────────────────────

    #[test]
    fn nonexistent_path_is_skipped() {
        let dir = tempdir().expect("temp dir");
        let missing = dir.path().join("book.epub"); // never created
        let path = missing.to_string_lossy().to_string();

        let result = collect_files(std::slice::from_ref(&path), false);
        assert!(result.is_empty());
    }

    // ── plain file inputs ────────────────────────────────────────────────────

    #[test]
    fn unsupported_extension_file_is_still_passed_through() {
        // Extension filtering only applies inside directories; a file given
        // directly on the CLI is always included regardless of its extension.
        let dir = tempdir().expect("temp dir");
        let file = dir.path().join("notes.txt");
        fs::write(&file, b"").expect("write");
        let path = file.to_string_lossy().to_string();

        let result = collect_files(std::slice::from_ref(&path), false);
        assert_eq!(
            result,
            vec![path],
            "unsupported-extension file should pass through"
        );
    }

    #[test]
    fn single_file_is_returned_as_is() {
        let dir = tempdir().expect("temp dir");
        let file = dir.path().join("book.epub");
        fs::write(&file, b"").expect("write");
        let path = file.to_string_lossy().to_string();

        let result = collect_files(std::slice::from_ref(&path), false);
        assert_eq!(result, vec![path]);
    }

    #[test]
    fn single_file_returned_even_with_recursive_flag() {
        let dir = tempdir().expect("temp dir");
        let file = dir.path().join("book.pdf");
        fs::write(&file, b"").expect("write");
        let path = file.to_string_lossy().to_string();

        let result = collect_files(std::slice::from_ref(&path), true);
        assert_eq!(result, vec![path]);
    }

    #[test]
    fn multiple_file_inputs_are_all_returned() {
        let dir = tempdir().expect("temp dir");
        let a = dir.path().join("a.epub");
        let b = dir.path().join("b.mobi");
        fs::write(&a, b"").expect("write a");
        fs::write(&b, b"").expect("write b");

        let inputs = vec![
            a.to_string_lossy().to_string(),
            b.to_string_lossy().to_string(),
        ];
        let result = collect_files(&inputs, false);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&a.to_string_lossy().to_string()));
        assert!(result.contains(&b.to_string_lossy().to_string()));
    }

    // ── directory without recursive flag ────────────────────────────────────

    #[test]
    fn directory_without_recursive_is_skipped() {
        let dir = tempdir().expect("temp dir");
        fs::write(dir.path().join("book.epub"), b"").expect("write");
        let dir_path = dir.path().to_string_lossy().to_string();

        let result = collect_files(&[dir_path], false);
        assert!(result.is_empty(), "expected empty; got {result:?}");
    }

    // ── recursive directory traversal ────────────────────────────────────────

    #[test]
    fn recursive_collects_supported_files() {
        let dir = tempdir().expect("temp dir");
        let sub = dir.path().join("subdir");
        fs::create_dir(&sub).expect("mkdir");

        fs::write(dir.path().join("a.epub"), b"").expect("write");
        fs::write(sub.join("b.mobi"), b"").expect("write");
        fs::write(sub.join("c.pdf"), b"").expect("write");
        fs::write(dir.path().join("readme.txt"), b"").expect("write");

        let dir_path = dir.path().to_string_lossy().to_string();
        let result = collect_files(&[dir_path], true);
        assert_eq!(
            result.len(),
            3,
            "expected 3 supported files; got {result:?}"
        );
    }

    #[test]
    fn recursive_excludes_unsupported_extensions() {
        let dir = tempdir().expect("temp dir");
        fs::write(dir.path().join("notes.txt"), b"").expect("write");
        fs::write(dir.path().join("image.png"), b"").expect("write");

        let dir_path = dir.path().to_string_lossy().to_string();
        let result = collect_files(&[dir_path], true);
        assert!(result.is_empty(), "expected empty; got {result:?}");
    }

    #[test]
    fn recursive_empty_directory_returns_empty() {
        let dir = tempdir().expect("temp dir");
        let dir_path = dir.path().to_string_lossy().to_string();
        let result = collect_files(&[dir_path], true);
        assert!(result.is_empty());
    }

    // ── mixed file and directory inputs ──────────────────────────────────────

    #[test]
    fn mixed_file_and_directory_inputs() {
        let dir = tempdir().expect("temp dir");
        let sub = dir.path().join("ebooks");
        fs::create_dir(&sub).expect("mkdir");

        let explicit_file = dir.path().join("explicit.pdf");
        fs::write(&explicit_file, b"").expect("write");
        fs::write(sub.join("nested.epub"), b"").expect("write nested");

        let inputs = vec![
            explicit_file.to_string_lossy().to_string(),
            sub.to_string_lossy().to_string(),
        ];
        let result = collect_files(&inputs, true);
        assert_eq!(result.len(), 2, "expected 2 files; got {result:?}");
        assert!(result.contains(&explicit_file.to_string_lossy().to_string()));
    }

    // ── extension case-insensitivity ─────────────────────────────────────────

    #[test]
    fn uppercase_extension_is_included() {
        let dir = tempdir().expect("temp dir");
        let file = dir.path().join("BOOK.EPUB");
        fs::write(&file, b"").expect("write");

        let dir_path = dir.path().to_string_lossy().to_string();
        let result = collect_files(&[dir_path], true);
        assert_eq!(result.len(), 1, "expected 1 file; got {result:?}");
    }

    // ── traversal order ──────────────────────────────────────────────────────

    #[test]
    fn directory_contents_returned_in_alphabetical_order() {
        // Verifies that sort_by_file_name() is in effect: files within a
        // directory must come back sorted regardless of creation order.
        let dir = tempdir().expect("temp dir");
        fs::write(dir.path().join("c.epub"), b"").expect("write c");
        fs::write(dir.path().join("a.epub"), b"").expect("write a");
        fs::write(dir.path().join("b.pdf"), b"").expect("write b");

        let dir_path = dir.path().to_string_lossy().to_string();
        let result = collect_files(&[dir_path.clone()], true);

        let expected = vec![
            dir.path().join("a.epub").to_string_lossy().to_string(),
            dir.path().join("b.pdf").to_string_lossy().to_string(),
            dir.path().join("c.epub").to_string_lossy().to_string(),
        ];
        assert_eq!(result, expected, "files must be sorted alphabetically");
    }

    #[test]
    fn mixed_inputs_preserve_input_order_then_sorted_dir_contents() {
        // Verifies the two-level ordering contract:
        //   1. Contributions appear in the order their inputs appear on the CLI.
        //   2. Within a directory expansion, entries are sorted by file name.
        let dir = tempdir().expect("temp dir");
        let sub = dir.path().join("books");
        fs::create_dir(&sub).expect("mkdir");

        let explicit = dir.path().join("z_explicit.pdf");
        fs::write(&explicit, b"").expect("write explicit");
        fs::write(sub.join("b.epub"), b"").expect("write b");
        fs::write(sub.join("a.mobi"), b"").expect("write a");

        let inputs = vec![
            explicit.to_string_lossy().to_string(),
            sub.to_string_lossy().to_string(),
        ];
        let result = collect_files(&inputs, true);

        let expected = vec![
            explicit.to_string_lossy().to_string(),
            sub.join("a.mobi").to_string_lossy().to_string(),
            sub.join("b.epub").to_string_lossy().to_string(),
        ];
        assert_eq!(
            result, expected,
            "explicit file must come first, then dir contents in alphabetical order"
        );
    }
}
