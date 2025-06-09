// tests/filter.rs

use roxide::{Cli, filter::PathFilter};

#[test]
fn test_filter_path() {
    // Explicit type annotations to avoid E0282
    let cli_with_pattern = Cli {
        recursive: false,
        pattern: Some("test".to_string()),
        dir: false,
        ..Default:: default()
    };

    let cli_no_pattern = Cli {
        recursive: false,
        pattern: None,
        dir: false,
        ..Default:: default()
    };

    // Should match path containing "test"
    assert!(PathFilter::filter("mytestfile.txt", &cli_with_pattern));

    // Should not match path without "test"
    assert!(!PathFilter::filter("myfile.txt", &cli_with_pattern));

    // Without pattern, always true
    assert!(PathFilter::filter("anything.txt", &cli_no_pattern));
}