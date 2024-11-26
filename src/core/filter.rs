#![allow(unused_labels)]

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use log::*;
use walkdir::{DirEntry, WalkDir};

use crate::show_error;

use super::args::Cli;
use super::rm::RoError;

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// # filter_paths
///
/// it will take the arguments [ recursive and pattern ]
/// and returns items to delete in a vec of PathBuf
///
pub fn filter_paths(items: Vec<PathBuf>, args: &Cli) -> RoError<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();
    trace!("{:?}", &items);
    for item in &items {
        if item.exists() {
            let condition = |filename: &OsStr| {
                filename
                    .to_str()
                    .map(|f| f.contains(args.pattern.as_deref().unwrap_or(""))) 
                    .unwrap_or(false)
            };
            match (args.recursive, args.pattern.is_some()) {
                (true, true) => {
                    for entry in item {
                        let walker = WalkDir::new(entry).into_iter();
                        for entry in walker.into_iter().filter_entry(|e| !is_hidden(e)) {
                            let entry = entry.context("no file matching the pattern").unwrap();
                            if entry.path().is_dir() {
                                continue;
                            }
                            let file_name = entry.path().file_name().unwrap();
                            if condition(file_name) {
                                trace!("{}", entry.path().display());
                                files.push(entry.path().to_path_buf())
                            }
                        }
                    }
                }
                (true, false) => files.push(item.to_path_buf()),
                (false, true) => {
                    if item.is_file() {
                        let filename = item.file_name().unwrap();
                        if condition(filename) {
                            trace!("{}", &item.display());
                            files.push(item.to_path_buf())
                        }
                    } else {
                        'iter_dir_contents: for entry in fs::read_dir(item)? {
                            let entry = entry?;
                            let path = entry.path();
                            if path.is_file() {
                                let file_name = path.file_name().unwrap();
                                if condition(file_name) {
                                    trace!("{:?}", &entry.path().display());
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
            show_error!(
                "cannot remove '{}': no such file or directory",
                &item.display()
            );
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
