use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

pub fn filter_files_by_pattern(items: Vec<PathBuf>, pattern: String) -> Vec<DirEntry> {
    // first iteration to to get get items from command
    for item in items {
        if !item.exists() {
            eprintln!(
                "roxide: cannot remove '{}': no such file or directory",
                item.display()
            );
            continue;
        }
        if !item.is_dir() {
            eprintln!("roxide: skipping directory '{}'", item.display());
            continue;
        }
        let walker = WalkDir::new(&item).into_iter();
        let mut filtered_items = Vec::new();
        for entry in walker.into_iter().filter_entry(|e| !is_hidden(e)) {
            let entry = entry.expect("roxide: cannot remove '{}': no such file or directory");
            if entry.path().is_dir() {
                continue;
            }
            let condition = entry
                .file_name()
                .to_str()
                .map(|m| m.contains(&pattern.to_string()))
                .unwrap_or(false);
            if condition {
                // println!("{}", entry.path().display());
                filtered_items.push(entry);
            }
        }
        return filtered_items;
    }
    let no_items: Vec<DirEntry> = Vec::new();
    no_items
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
