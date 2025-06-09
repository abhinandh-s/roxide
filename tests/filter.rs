// tests/filter.rs
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

use anyhow::Result;
use roxide::args::{filter::PathFilter, Cli, RoxError};

/// Helper macro to create a Cli instance with minimal required args,
/// avoiding repeated boilerplate when testing different flags.
macro_rules! make_cli {
    (
        recursive: $recursive:expr,
        pattern: $pattern:expr,
        dir: $dir:expr
    ) => {
        Cli {
            file: None,
            recursive: $recursive,
            list: false,
            interactive: None,
            pattern: $pattern.map(|s| s.to_string()),
            force: None,
            verbose: false,
            dir: $dir,
            #[cfg(feature = "extra_commands")]
            check: false,
            command: None,
        }
    };
}

/// Creates a file at the specified path with some dummy content.
fn create_file(path: &Path) {
    let mut f = File::create(path).expect("Failed to create file");
    writeln!(f, "dummy content").expect("Failed to write to file");
}

/// Creates a hidden file (starting with '.') at the specified path.
fn create_hidden_file(path: &Path) {
    let hidden_path = path.with_file_name(format!(".{}", path.file_name().unwrap().to_string_lossy()));
    create_file(&hidden_path);
}

#[test]
/// Tests `filter` with a non-recursive call on a single file without pattern.
/// Should return the file itself in the result vector.
fn test_filter_single_file_no_pattern() -> Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("file.txt");
    create_file(&file_path);

    let cli = make_cli!(recursive: false, pattern: None, dir: false);
    let result = PathFilter::filter(vec![file_path.clone()], &cli)?;

    assert_eq!(result, vec![file_path]);
    Ok(())
}

#[test]
/// Tests non-recursive directory removal with `dir = true`.
/// Should include the directory itself if `dir` flag is set.
fn test_filter_non_recursive_dir_with_dir_flag() -> Result<()> {
    let dir = tempdir()?;
    let sub_dir = dir.path().join("subdir");
    fs::create_dir(&sub_dir)?;

    let cli = make_cli!(recursive: false, pattern: None, dir: true);
    let result = PathFilter::filter(vec![sub_dir.clone()], &cli)?;

    assert_eq!(result, vec![sub_dir]);
    Ok(())
}

#[test]
/// Tests non-recursive directory removal without `dir` flag.
/// Should NOT include the directory and show error (captured manually).
fn test_filter_non_recursive_dir_without_dir_flag() -> Result<()> {
    let dir = tempdir()?;
    let sub_dir = dir.path().join("subdir");
    fs::create_dir(&sub_dir)?;

    let cli = make_cli!(recursive: false, pattern: None, dir: false);
    let result = PathFilter::filter(vec![sub_dir.clone()], &cli);

    // Directory is not included, error is logged via show_error macro internally.
    // We expect Ok with empty vec or error handling.
    assert!(result.is_ok());
    let files = result.unwrap();
    assert!(files.is_empty());
    Ok(())
}

#[test]
/// Tests recursive pattern matching: only files matching pattern get filtered.
/// Should descend into directory tree and exclude hidden files.
fn test_filter_recursive_with_pattern() -> Result<()> {
    let dir = tempdir()?;
    let sub_dir = dir.path().join("subdir");
    fs::create_dir(&sub_dir)?;

    let file1 = sub_dir.join("keep.pdf");
    let file2 = sub_dir.join("skip.txt");
    create_file(&file1);
    create_file(&file2);

    // hidden file, should be skipped
    create_hidden_file(&sub_dir.join("hidden.pdf"));

    let cli = make_cli!(recursive: true, pattern: Some(".pdf"), dir: false);
    let result = PathFilter::filter(vec![sub_dir.clone()], &cli)?;

    assert!(result.contains(&file1));
    assert!(!result.contains(&file2));
    // hidden files excluded
    assert_eq!(result.len(), 1);
    Ok(())
}

#[test]
/// Tests non-recursive with pattern on a directory: should only match files directly inside.
/// Should exclude directories and hidden files.
fn test_filter_non_recursive_with_pattern_dir() -> Result<()> {
    let dir = tempdir()?;
    let sub_dir = dir.path().join("subdir");
    fs::create_dir(&sub_dir)?;

    let file1 = sub_dir.join("keep.pdf");
    let file2 = sub_dir.join("skip.txt");
    create_file(&file1);
    create_file(&file2);
    create_hidden_file(&sub_dir.join("hidden.pdf"));

    let cli = make_cli!(recursive: false, pattern: Some(".pdf"), dir: false);
    let result = PathFilter::filter(vec![sub_dir.clone()], &cli)?;

    assert!(result.contains(&file1));
    assert!(!result.contains(&file2));
    assert_eq!(result.len(), 1);
    Ok(())
}

#[test]
/// Tests that pattern with no matches returns an error of type PatternNoMatch.
fn test_filter_pattern_no_match_error() -> Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("file.txt");
    create_file(&file_path);

    let cli = make_cli!(recursive: false, pattern: Some(".pdf"), dir: false);
    let err = PathFilter::filter(vec![file_path], &cli).unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("PatternNoMatch"));
    Ok(())
}

#[test]
/// Tests filtering on a non-existent path: should skip and not panic.
fn test_filter_non_existent_path() -> Result<()> {
    let non_existent = PathBuf::from("/non/existent/path");

    let cli = make_cli!(recursive: false, pattern: None, dir: false);
    let result = PathFilter::filter(vec![non_existent], &cli)?;

    // Should just return empty vec, no panic
    assert!(result.is_empty());
    Ok(())
}

#[test]
/// Tests the matches_pattern utility function directly.
/// Should detect if filename contains the pattern substring.
fn test_matches_pattern() {
    let cli = make_cli!(recursive: false, pattern: Some(".txt"), dir: false);
    let file_name = std::ffi::OsStr::new("file.txt");
    let no_match = std::ffi::OsStr::new("file.pdf");

    assert!(PathFilter::matches_pattern(&cli, file_name));
    assert!(!PathFilter::matches_pattern(&cli, no_match));

    // Pattern None always returns false
    let cli_no_pattern = make_cli!(recursive: false, pattern: None, dir: false);
    assert!(!PathFilter::matches_pattern(&cli_no_pattern, file_name));
}

#[test]
/// Tests that hidden files are properly detected and filtered out in recursive walks.
fn test_is_hidden_filtering() -> Result<()> {
    use walkdir::DirEntry;
    use std::fs;

    let dir = tempdir()?;
    let hidden_file = dir.path().join(".hiddenfile");
    create_file(&hidden_file);

    let walker = walkdir::WalkDir::new(dir.path());
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == ".hiddenfile" {
            assert!(PathFilter::is_hidden(&entry));
        }
    }
    Ok(())
}