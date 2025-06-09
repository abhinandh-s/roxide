use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use tempdir::TempDir;

use roxide::filter::PathFilter;
use roxide::Cli;

/// Helper macro to setup test environment:
/// - Accepts file and dir definitions
/// - CLI config inline
macro_rules! setup_test {
    (
        files: [ $( $fname:expr => $fcontent:expr ),* $(,)? ],
        dirs: [ $( $dname:expr ),* $(,)? ],
        cli: {
            pattern: $pattern:expr,
            recursive: $recursive:expr,
            dir: $dir_flag:expr,
            items: [ $( $item:expr ),* $(,)? ]
        }
    ) => {{
        let tmp = TempDir::new("test_filter").expect("tempdir failed");
        let base = tmp.path().to_path_buf();

        // Create dirs
        $(
            fs::create_dir_all(base.join($dname)).unwrap();
        )*

        // Create files
        $(
            let full_path = base.join($fname);
            let mut f = File::create(&full_path).unwrap();
            writeln!(f, "{}", $fcontent).unwrap();
        )*

        let cli = Cli {
            pattern: $pattern.map(|s| s.to_string()),
            recursive: $recursive,
            dir: $dir_flag,
            dry: false,
            force: false,
            items: vec![ $( base.join($item) ),* ],
        };

        (tmp, base, cli)
    }};
}

#[test]
fn test_file_no_pattern_no_recursive() {
    let (_tmp, _base, args) = setup_test!(
        files: ["file.txt" => "abc"],
        dirs: [],
        cli: {
            pattern: None,
            recursive: false,
            dir: false,
            items: ["file.txt"]
        }
    );

    let result = PathFilter::filter(args.items.clone(), &args).unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_file_with_matching_pattern() {
    let (_tmp, _base, args) = setup_test!(
        files: ["match.log" => "abc"],
        dirs: [],
        cli: {
            pattern: Some("match"),
            recursive: false,
            dir: false,
            items: ["match.log"]
        }
    );

    let result = PathFilter::filter(args.items.clone(), &args).unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_file_with_non_matching_pattern() {
    let (_tmp, _base, args) = setup_test!(
        files: ["random.txt" => "abc"],
        dirs: [],
        cli: {
            pattern: Some("nope"),
            recursive: false,
            dir: false,
            items: ["random.txt"]
        }
    );

    let result = PathFilter::filter(args.items.clone(), &args);
    assert!(result.is_err());
}

#[test]
fn test_hidden_file_skipped_in_recursive() {
    let (_tmp, _base, args) = setup_test!(
        files: ["sub/.hidden.txt" => "secret"],
        dirs: ["sub"],
        cli: {
            pattern: Some("hidden"),
            recursive: true,
            dir: false,
            items: ["sub"]
        }
    );

    let result = PathFilter::filter(args.items.clone(), &args).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_recursive_matching() {
    let (_tmp, _base, args) = setup_test!(
        files: [
            "dir/log1.txt" => "yes",
            "dir/ignore.txt" => "no"
        ],
        dirs: ["dir"],
        cli: {
            pattern: Some("log"),
            recursive: true,
            dir: false,
            items: ["dir"]
        }
    );

    let result = PathFilter::filter(args.items.clone(), &args).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].to_string_lossy().contains("log1.txt"));
}

#[test]
fn test_non_recursive_dir_pattern_match() {
    let (_tmp, _base, args) = setup_test!(
        files: [
            "sub/keep.log" => "match",
            "sub/nothing.txt" => "no"
        ],
        dirs: ["sub"],
        cli: {
            pattern: Some("keep"),
            recursive: false,
            dir: false,
            items: ["sub"]
        }
    );

    let result = PathFilter::filter(args.items.clone(), &args).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].to_string_lossy().contains("keep.log"));
}

#[test]
fn test_dir_allowed_with_dir_flag() {
    let (_tmp, base, args) = setup_test!(
        files: [],
        dirs: ["target"],
        cli: {
            pattern: None,
            recursive: false,
            dir: true,
            items: ["target"]
        }
    );

    let result = PathFilter::filter(args.items.clone(), &args).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], base.join("target"));
}

#[test]
fn test_dir_disallowed_without_dir_flag() {
    let (_tmp, _base, args) = setup_test!(
        files: [],
        dirs: ["test_dir"],
        cli: {
            pattern: None,
            recursive: false,
            dir: false,
            items: ["test_dir"]
        }
    );

    let result = PathFilter::filter(args.items.clone(), &args).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_nonexistent_file_is_skipped() {
    let (_tmp, _base, mut args) = setup_test!(
        files: [],
        dirs: [],
        cli: {
            pattern: None,
            recursive: false,
            dir: false,
            items: []
        }
    );

    args.items = vec![PathBuf::from("/tmp/this/does/not/exist.txt")];

    let result = PathFilter::filter(args.items.clone(), &args).unwrap();
    assert!(result.is_empty());
}