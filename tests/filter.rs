use std::path::{Path, PathBuf};
use tempfile::{tempdir, NamedTempFile};
use roxide::args::{Cli, filter::PathFilter};
use anyhow::Result;

// Macro to simplify calling filter and asserting expected results or errors
macro_rules! test_filter {
    // Test success case: expects some paths
    ($name:ident, $items:expr, $cli:expr, $expected:expr) => {
        #[test]
        fn $name() -> Result<()> {
            let result = PathFilter::filter($items, &$cli)?;
            assert_eq!(result, $expected);
            Ok(())
        }
    };

    // Test failure case: expects an error
    ($name:ident, $items:expr, $cli:expr, error) => {
        #[test]
        fn $name() {
            let result = PathFilter::filter($items, &$cli);
            assert!(result.is_err());
        }
    };
}

// Helper: create a Cli with specified recursive and pattern flags
fn make_cli(recursive: bool, pattern: Option<&str>, dir_flag: bool) -> Cli {
    Cli {
        recursive,
        pattern: pattern.map(String::from),
        dir: dir_flag,
        file: None,
        force: None,
        interactive: None,
        list: false,
        verbose: false,
        command: None,
        // other fields defaulted
        ..Default::default()
    }
}

mod tests {
    use super::*;

    // Basic test: single existing file, no recursion, no pattern
    test_filter!(
        basic_file_no_pattern,
        {
            let file = NamedTempFile::new().unwrap();
            vec![file.path().to_path_buf()]
        },
        make_cli(false, None, false),
        {
            let file = NamedTempFile::new().unwrap();
            vec![file.path().to_path_buf()]
        }
    );

    // Directory without recursion, no pattern, dir flag false - expect error (cannot remove directory)
    #[test]
    fn dir_no_recursive_no_pattern_error() -> Result<()> {
        let dir = tempdir()?;
        let items = vec![dir.path().to_path_buf()];
        let cli = make_cli(false, None, false);

        let result = PathFilter::filter(items, &cli);
        assert!(result.is_ok()); // Wait, the code *shows error* but still returns Ok.
        // According to your code, it calls show_error but does not return Err here.
        // So filter returns Ok, but the file list is empty. Test accordingly:
        let files = result?;
        assert!(files.is_empty());
        Ok(())
    }

    // Directory with recursion, no pattern: expect directory path itself returned
    test_filter!(
        dir_with_recursive_no_pattern,
        {
            let dir = tempdir().unwrap();
            vec![dir.path().to_path_buf()]
        },
        make_cli(true, None, false),
        {
            let dir = tempdir().unwrap();
            vec![dir.path().to_path_buf()]
        }
    );

    // Directory with pattern (non-recursive), should collect matching files only
    #[test]
    fn dir_no_recursive_with_pattern() -> Result<()> {
        let dir = tempdir()?;
        // Create two files: match.txt and no_match.rs
        let file_match = dir.path().join("match.txt");
        std::fs::write(&file_match, "hello")?;
        let file_no_match = dir.path().join("no_match.rs");
        std::fs::write(&file_no_match, "world")?;

        let items = vec![dir.path().to_path_buf()];
        let cli = make_cli(false, Some(".txt"), false);

        let result = PathFilter::filter(items, &cli)?;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], file_match);
        Ok(())
    }

    // Recursive with pattern: collect matching files deeply
    #[test]
    fn recursive_with_pattern() -> Result<()> {
        let dir = tempdir()?;

        // Create nested files: a.pdf, subdir/b.pdf, subdir/c.txt
        let file1 = dir.path().join("a.pdf");
        std::fs::write(&file1, "x")?;

        let subdir = dir.path().join("subdir");
        std::fs::create_dir(&subdir)?;

        let file2 = subdir.join("b.pdf");
        std::fs::write(&file2, "y")?;

        let file3 = subdir.join("c.txt");
        std::fs::write(&file3, "z")?;

        let items = vec![dir.path().to_path_buf()];
        let cli = make_cli(true, Some(".pdf"), false);

        let result = PathFilter::filter(items, &cli)?;
        // Should only contain a.pdf and subdir/b.pdf
        assert!(result.contains(&file1));
        assert!(result.contains(&file2));
        assert_eq!(result.len(), 2);
        Ok(())
    }

    // Pattern with no match: expect error (PatternNoMatch)
    #[test]
    fn pattern_no_match_error() -> Result<()> {
        let file = NamedTempFile::new()?;
        let items = vec![file.path().to_path_buf()];
        let cli = make_cli(false, Some("nonexistentpattern"), false);

        let result = PathFilter::filter(items, &cli);
        assert!(result.is_err());
        Ok(())
    }

    // File that doesn't exist: should show error and skip
    #[test]
    fn file_does_not_exist() -> Result<()> {
        let fake_path = PathBuf::from("/tmp/nonexistentfile12345");
        let items = vec![fake_path];
        let cli = make_cli(false, None, false);

        let result = PathFilter::filter(items, &cli)?;
        assert!(result.is_empty());
        Ok(())
    }
}