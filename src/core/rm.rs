#![allow(unused_labels)]
#![allow(unused_imports)]

use std::env::current_dir;
use std::ffi::OsStr;
use std::{fs, io, usize};
use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};

use anyhow::Context;
use log::*;
use walkdir::{DirEntry, WalkDir};

use crate::core::filter::filter_paths;
use crate::core::trash::Trash;
use crate::revert::write_log;
use crate::utils::trash_dir;
use crate::{dev_debug, show_error, verbose};

use super::args::{Cli, InteractiveMode};


pub fn handle_interactive(items: Vec<PathBuf>, args: &Cli) {
    if args.interactive == Some(InteractiveMode::Once) && (args.recursive || items.len() > 3) {
        println!("remove 1 argument recursively? (y / n) ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        trace!("You guessed: {}", guess);
    }

    match args.interactive {
        Some(InteractiveMode::Never) => {}
        Some(InteractiveMode::Once) => {}
        Some(InteractiveMode::Always) => {}
        Some(InteractiveMode::PromptProtected) => {}
        None => {}
    }
    
}


pub fn core_remove(items: Vec<PathBuf>, args: &Cli) -> anyhow::Result<(), anyhow::Error> {
    let f = filter_paths(items.clone(), args).unwrap();
    handle_interactive(items, args);
    trace!("{:#?}", f);
    Ok(())
}

//#[allow(unused_labels)]
//pub fn prev_core_remove(
//    items: Vec<PathBuf>,
//    args: &Cli,
//) -> anyhow::Result<(), anyhow::Error> {
//    verbose!(args.verbose, "recursive: {:?}", args.recursive);
//    'iter_through_arg_items: for item in items {
//        if item.exists() {
//            let exact_file_name = item.file_name().unwrap();
//            trace!("exact_file_name: {}", exact_file_name.to_string_lossy());
//            if args.recursive {
//                // recursive remove
//                eprintln!("{} is a directory.\nTry: roxide -r", item.display());
//            } else if item.is_dir() {
//                // if we got a directory in non-recursive remove
//                eprintln!("{} is a directory.\nTry: roxide -r", item.display());
//                continue;
//            } else {
//                // non-recursive remove
//                eprintln!("{} is a directory.\nTry: roxide -r", item.display());
//            }
//        } else {
//            eprintln!(
//                "roxide: cannot remove '{}': no such file or directory",
//                &item.display()
//            );
//            continue;
//        }
//    }
//    Ok(())
//}
//
//
//#[allow(clippy::cognitive_complexity)]
//fn non_recursive_pattern_matching(
//    items: Vec<PathBuf>,
//    pattern: Option<String>,
//) -> anyhow::Result<(), anyhow::Error> {
//    trace!("hello from non_recursive_pattern_matching");
//    let pat = pattern.to_owned().unwrap();
//    let p = pat.as_str();
//    trace!("pattern for matching: {:?}", p);
//    'iter_arg_items: for item in items {
//        if item.exists() {
//            'iter_dir_contents: for entry in fs::read_dir(&item)? {
//                let entry = entry?;
//                let path = entry.path();
//                if path.is_file() {
//                    let file_name = path.file_name().unwrap();
//                    let condition = file_name.to_str().map(|f| f.contains(p)).unwrap_or(false);
//                    if condition {
//                        let file_name = path.file_name().unwrap();
//                        let p = Path::new(file_name);
//                        let trash = Trash { file: p };
//                        // let id = trash.get_log_id();
//                        let item_path = current_dir()
//                            .unwrap()
//                            .join(&item)
//                            .join(path.file_name().unwrap());
//                        // let trash_path = trash_dir().join(trash.trash_name(id.1));
//                        fs::rename(
//                            &item_path,
//                            trash_dir().join(trash.trash_name(trash.get_log_id().1)),
//                        )
//                        .unwrap();
//                    }
//                } else {
//                    trace!("skipping directory: {:?}", path);
//                }
//            }
//        } else {
//            eprintln!(
//                "roxide: cannot remove '{}': no such file or directory",
//                &item.display()
//            );
//            return Ok(());
//        }
//    }
//    Ok(())
//}
//
///// remove_files
///// the core function of the whole crate
///// accepts arguments as Vec<PathBuf> so we can take multiple dirs/files in single command
///// eg: roxide -r some/dir another/dir
/////
///// takes Vec<PathBuf> and flatten all the
//pub fn remove_files(items: Vec<PathBuf>, args: &Cli) -> anyhow::Result<()> {
//    match (args.pattern.is_some(), args.recursive) {
//        (true, false) => {
//            non_recursive_pattern_matching(items, args.pattern.clone()).unwrap();
//        }
//        (true, true) => {
//            let p = args.pattern.to_owned().unwrap();
//            let filtered_items = filter_files_by_pattern(items, p);
//            for item in filtered_items {
//                let file_name = item.path().file_name().unwrap();
//                trace!("file_name: {:?}", file_name);
//                let p = Path::new(file_name);
//                let trash = Trash { file: p };
//                let id = trash.get_log_id();
//                trace!("id: {:?}", id);
//                let item_path = current_dir().unwrap().join(item.path());
//                trace!("item path: {:?}", item_path);
//                let trash_path = trash_dir().join(trash.trash_name(id.1));
//                trace!("trash path: {:?}", trash_path);
//                fs::rename(
//                    &item_path,
//                    trash_dir().join(trash.trash_name(trash.get_log_id().1)),
//                )
//                .unwrap();
//            }
//        }
//        (false, true) => {
//            for item in items {
//                if item.exists() {
//                    let item = item.as_path();
//                    let trash = Trash { file: item };
//                    let id = trash.get_log_id();
//                    trace!("id: {:?}", id);
//                    let item_path = current_dir().unwrap().join(item);
//                    trace!("item path recursive true: {:?}", item_path);
//                    let trash_path = trash_dir().join(trash.trash_name(id.1));
//                    trace!("trash path recursive true: {:?}", trash_path);
//                    fs::rename(
//                        &item_path,
//                        trash_dir().join(trash.trash_name(trash.get_log_id().1)),
//                    )
//                    .unwrap();
//                    verbose!(
//                        args.verbose,
//                        "Trashed {} to {}",
//                        item.display(),
//                        trash_dir()
//                            .join(trash.trash_name(trash.get_log_id().1))
//                            .display()
//                    );
//                    write_log(
//                        id.0,
//                        item_path.to_str().unwrap().to_string(),
//                        trash_path.to_str().unwrap().to_string(),
//                    )
//                    .unwrap();
//                } else {
//                    eprintln!(
//                        "roxide: cannot remove '{}': no such file or directory",
//                        &item.display()
//                    );
//                    return Ok(());
//                }
//            }
//        }
//        (false, false) => {
//            for item in items {
//                if item.exists() {
//                    if item.is_dir() {
//                        eprintln!("{} is a directory.\nTry: roxide -r", item.display());
//                        return Ok(());
//                    }
//                    let item = item.as_path();
//                    let trash = Trash { file: item };
//                    let id = trash.get_log_id();
//                    trace!("id: {:?}", id);
//                    let item_path = current_dir().unwrap().join(item);
//                    let trash_path = trash_dir().join(trash.trash_name(id.1));
//                    trace!("trash path: {:?}", trash_path);
//                    fs::rename(
//                        &item_path,
//                        trash_dir().join(trash.trash_name(trash.get_log_id().1)),
//                    )
//                    .unwrap();
//                    if args.verbose {
//                        println!(
//                            "Trashed {} to {}",
//                            item.display(),
//                            trash_dir()
//                                .join(trash.trash_name(trash.get_log_id().1))
//                                .display()
//                        )
//                    }
//                    write_log(
//                        id.0,
//                        item_path.to_str().unwrap().to_string(),
//                        trash_path.to_str().unwrap().to_string(),
//                    )
//                    .unwrap();
//                } else {
//                    eprintln!(
//                        "roxide: cannot remove '{}': no such file or directory",
//                        &item.display()
//                    );
//                    return Ok(());
//                }
//            }
//        }
//    }
//    Ok(())
//}
