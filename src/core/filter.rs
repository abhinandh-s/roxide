#![allow(unused_labels)]

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use log::*;
use roxide::show_error;
use walkdir::{DirEntry, WalkDir};

use super::args::Cli;
use super::rm::RoError;

#[derive(Debug)]
pub struct PathFilter;

impl PathFilter {
    /// if pattern matches it will return true
    pub fn matches_pattern(args: &Cli, filename: &OsStr) -> bool {
        args.pattern
            .as_deref()
            .map(|pat| filename.to_str().is_some_and(|f| f.contains(pat)))
            .unwrap_or(false)
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    fn check_exists(item: PathBuf) -> bool {
        item.exists() || {
            // its OR not a colosure
            show_error!(
                "cannot remove '{}': no such file or directory",
                &item.display()
            );
            false
        }
    }

    /// # filter_paths
    ///
    /// it will take the arguments [ recursive and pattern ]
    /// and returns items to delete in a vec of PathBuf
    ///
    pub fn init(items: Vec<PathBuf>, args: &Cli) -> RoError<Vec<PathBuf>> {
        let mut files: Vec<PathBuf> = Vec::new();
        for item in &items {
            if Self::check_exists(item.into()) {
                match (args.recursive, args.pattern.is_some()) {
                    (true, true) => {
                        for entry in item {
                            let walker = WalkDir::new(entry).into_iter();
                            for entry in walker.into_iter().filter_entry(|e| !Self::is_hidden(e)) {
                                let entry = entry.unwrap();
                                if entry.path().is_dir() {
                                    continue;
                                }
                                let filename = entry.path().file_name().unwrap();
                                if Self::matches_pattern(args, filename) {
                                    files.push(entry.path().to_path_buf())
                                }
                            }
                        }
                    }
                    (true, false) => files.push(item.to_path_buf()),
                    (false, true) => {
                        if item.is_file() {
                            let filename = item.file_name().unwrap();
                            if Self::matches_pattern(args, filename) {
                                files.push(item.to_path_buf())
                            }
                        } else {
                            'iter_dir_contents: for entry in fs::read_dir(item)? {
                                let entry = entry?;
                                let path = entry.path();
                                if path.is_file() {
                                    let filename = path.file_name().unwrap();
                                    if Self::matches_pattern(args, filename) {
                                        files.push(entry.path().to_path_buf())
                                    }
                                } else {
                                    trace!("skipping directory: {:?}", path);
                                }
                            }
                        }
                    }
                    (false, false) => {
                        if item.is_file() || args.dir {
                            files.push(item.to_path_buf())
                        } else {
                            show_error!("cannot remove '{}': Is a directory", item.display());
                        }
                    }
                }
            } else {
                continue;
            }
        }
        match &args.pattern {
            Some(pat) if files.is_empty() => {
                return Err(crate::core::error::Error::PatternNoMatch(pat.to_string()));
            }
            _ => {}
        }
        Ok(files)
    }
}
