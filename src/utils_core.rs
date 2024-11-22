#![allow(unused_labels)]
// when using sudo there is no trash dir for root to programs will fail for behave differenlty
// on my tests it moved files to home dir

use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Ok;
use log::*;

use crate::core::args::Cli;
use crate::pattern::filter_files_by_pattern;
use crate::revert::write_log;
use crate::utils::{current_time, trash_dir};

#[derive(Debug)]
pub struct Trash<'a> {
    pub file: &'a Path,
}

impl<'a> Trash<'a> {
    pub fn get_log_id(&self) -> (String, String) {
        (
            current_time().format("%Y%m%d%H%M%S").to_string(),
            current_time().format("%Y-%m-%d_%H:%M:%S").to_string(),
        )
    }
    pub fn trash_name(&self, log_id: String) -> String {
        let trash_file = trash_dir()
            .join(self.file)
            .try_exists()
            .expect("Cant check whether trash dir exists or not");
        error!(
            "trash_name from trash_name function: {}",
            self.file.file_stem().unwrap().to_str().unwrap()
        );
        let file_stem = self.file.file_stem().unwrap().to_str().unwrap();
        let file_ext = self.file.extension().and_then(|e| e.to_str());
        let file_name_to_check = self.file.file_name().unwrap();
        let trash_file_to_check = trash_dir().join(file_name_to_check).exists();
        let trash_file_name = |stem: &str, ext: Option<&str>| -> String {
            match ext {
                Some(e) => format!("{}.{}.{}", stem, log_id, e),
                None => format!("{}.{}", stem, log_id),
            }
        };

        if !trash_file_to_check {
            debug!(
                "impl Trash struct: {:#?}",
                self.file
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap()
            );
            // self.file.file_name().unwrap().to_str().unwrap().to_string()
            self.file
                .file_name()
                .map(|t| t.to_string_lossy().to_string())
                .expect("failed to set trash name")
        } else {
            let trash_name = trash_file_name(file_stem, file_ext);
            debug!(
                "Trash name from impl: {:#?}",
                self.file
                    .with_file_name(&trash_name)
                    .to_str()
                    .unwrap()
                    .to_string()
            );
            trash_name
        }
    }
}

#[allow(clippy::cognitive_complexity)]
fn non_recursive_pattern_matching(
    items: Vec<PathBuf>,
    pattern: Option<String>,
) -> anyhow::Result<(), anyhow::Error> {
    trace!("hello from non_recursive_pattern_matching");
    let pat = pattern.to_owned().unwrap();
    let p = pat.as_str();
    trace!("pattern for matching: {:?}", p);
    'iter_arg_items: for item in items {
        if item.exists() {
            'iter_dir_contents: for entry in fs::read_dir(&item)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap();
                    let condition = file_name.to_str().map(|f| f.contains(p)).unwrap_or(false);
                    if condition {
                        let file_name = path.file_name().unwrap();
                        let p = Path::new(file_name);
                        let trash = Trash { file: p };
                        // let id = trash.get_log_id();
                        let item_path = current_dir()
                            .unwrap()
                            .join(&item)
                            .join(path.file_name().unwrap());
                        // let trash_path = trash_dir().join(trash.trash_name(id.1));
                        fs::rename(
                            &item_path,
                            trash_dir().join(trash.trash_name(trash.get_log_id().1)),
                        )
                        .unwrap();
                    }
                } else {
                    trace!("skipping directory: {:?}", path);
                }
            }
        } else {
            eprintln!(
                "roxide: cannot remove '{}': no such file or directory",
                &item.display()
            );
            return Ok(());
        }
    }
    Ok(())
}

/// remove_files
/// the core function of the whole crate
/// accepts arguments as Vec<PathBuf> so we can take multiple dirs/files in single command
/// eg: roxide -r some/dir another/dir
///
/// takes Vec<PathBuf> and flatten all the
#[allow(unused_variables)]
pub fn remove_files(items: Vec<PathBuf>, args: &Cli) -> anyhow::Result<()> {
    match (args.pattern.is_some(), args.recursive) {
        (true, false) => {
            let pat = args.pattern.to_owned().unwrap();
            non_recursive_pattern_matching(items, args.pattern.clone()).unwrap();
        }
        (true, true) => {
            let p = args.pattern.to_owned().unwrap();
            let filtered_items = filter_files_by_pattern(items, p);
            for item in filtered_items {
                let file_name = item.path().file_name().unwrap();
                trace!("file_name: {:?}", file_name);
                let p = Path::new(file_name);
                let trash = Trash { file: p };
                let id = trash.get_log_id();
                trace!("id: {:?}", id);
                let item_path = current_dir().unwrap().join(item.path());
                trace!("item path: {:?}", item_path);
                let trash_path = trash_dir().join(trash.trash_name(id.1));
                trace!("trash path: {:?}", trash_path);
                fs::rename(
                    &item_path,
                    trash_dir().join(trash.trash_name(trash.get_log_id().1)),
                )
                .unwrap();
            }
        }
        (false, true) => {
            for item in items {
                if item.exists() {
                    let item = item.as_path();
                    let trash = Trash { file: item };
                    let id = trash.get_log_id();
                    trace!("id: {:?}", id);
                    let item_path = current_dir().unwrap().join(item);
                    trace!("item path recursive true: {:?}", item_path);
                    let trash_path = trash_dir().join(trash.trash_name(id.1));
                    trace!("trash path recursive true: {:?}", trash_path);
                    fs::rename(
                        &item_path,
                        trash_dir().join(trash.trash_name(trash.get_log_id().1)),
                    )
                    .unwrap();
                    if args.verbose {
                        println!(
                            "Trashed {} to {}",
                            item.display(),
                            trash_dir()
                                .join(trash.trash_name(trash.get_log_id().1))
                                .display()
                        )
                    }
                    write_log(
                        id.0,
                        item_path.to_str().unwrap().to_string(),
                        trash_path.to_str().unwrap().to_string(),
                    )
                    .unwrap();
                } else {
                    eprintln!(
                        "roxide: cannot remove '{}': no such file or directory",
                        &item.display()
                    );
                    return Ok(());
                }
            }
        }
        (false, false) => {
            for item in items {
                if item.exists() {
                    if item.is_dir() {
                        eprintln!("{} is a directory.\nTry: roxide -r", item.display());
                        return Ok(());
                    }
                    let item = item.as_path();
                    let trash = Trash { file: item };
                    let id = trash.get_log_id();
                    trace!("id: {:?}", id);
                    let item_path = current_dir().unwrap().join(item);
                    let trash_path = trash_dir().join(trash.trash_name(id.1));
                    trace!("trash path: {:?}", trash_path);
                    fs::rename(
                        &item_path,
                        trash_dir().join(trash.trash_name(trash.get_log_id().1)),
                    )
                    .unwrap();
                    if args.verbose {
                        println!(
                            "Trashed {} to {}",
                            item.display(),
                            trash_dir()
                                .join(trash.trash_name(trash.get_log_id().1))
                                .display()
                        )
                    }
                    write_log(
                        id.0,
                        item_path.to_str().unwrap().to_string(),
                        trash_path.to_str().unwrap().to_string(),
                    )
                    .unwrap();
                } else {
                    eprintln!(
                        "roxide: cannot remove '{}': no such file or directory",
                        &item.display()
                    );
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

//fn core_remove(
//    items: Vec<PathBuf>,
//    recursive: bool,
//    verbose: bool,
//) -> anyhow::Result<(), anyhow::Error> {
//    if verbose {
//        println!("recursive: {:?}", recursive);
//    }
//    'iter_through_arg_items: for item in items {
//        if item.exists() {
//            let exact_file_name = item.file_name().unwrap();
//            trace!("exact_file_name: {}", exact_file_name.to_string_lossy());
//            if recursive {
//                // recursive remove
//                eprintln!("{} is a directory.\nTry: roxide -r", item.display());
//            } else if item.is_dir() {
//                // if we got a directory in non-recursive remove
//                eprintln!("{} is a directory.\nTry: roxide -r", item.display());
//                return Ok(());
//            } else {
//                // non-recursive remove
//                eprintln!("{} is a directory.\nTry: roxide -r", item.display());
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
