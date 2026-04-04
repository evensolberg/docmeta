use walkdir::WalkDir;

use crate::utils;

const SUPPORTED_EXTENSIONS: &[&str] = &["epub", "mobi", "pdf"];

/// Collect file paths from the supplied inputs.
///
/// Each entry in `inputs` is treated as follows:
///
/// - **File**: included as-is (regardless of extension or `recursive`).
/// - **Directory** with `recursive = true`: walked depth-first; only files
///   whose extensions appear in [`SUPPORTED_EXTENSIONS`] are included.
/// - **Directory** with `recursive = false`: skipped with a warning.
/// - Anything that does not exist on disk: skipped with a warning.
///
/// The returned list preserves the encounter order (files before directory
/// contents, directory contents in `WalkDir` order).
pub fn collect_files(inputs: &[String], recursive: bool) -> Vec<String> {
    let mut result = Vec::new();

    for input in inputs {
        let Ok(meta) = std::fs::metadata(input) else {
            log::warn!("Path does not exist, skipping: {input}");
            continue;
        };

        if meta.is_file() {
            result.push(input.clone());
        } else if meta.is_dir() {
            if !recursive {
                log::warn!("Directory skipped (use --recursive to traverse): {input}");
                continue;
            }
            for entry in WalkDir::new(input)
                .into_iter()
                .filter_map(std::result::Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let path_str = entry.path().to_string_lossy();
                if SUPPORTED_EXTENSIONS.contains(&utils::get_extension(&path_str).as_str()) {
                    result.push(path_str.into_owned());
                }
            }
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
        let result = collect_files(&["/no/such/path/book.epub".to_string()], false);
        assert!(result.is_empty());
    }

    // ── plain file inputs ────────────────────────────────────────────────────

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
}
