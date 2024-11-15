use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use log::{info, trace};

use crate::revert::write_log;
use crate::utils::{current_time, trash_dir};

#[derive(Debug)]
struct Trash<'a> {
    file: &'a Path,
}

impl<'a> Trash<'a> {
    fn get_log_id(&self) -> String {
        current_time().format("%Y%m%d%H%M%S").to_string()
    }
    fn trash_name(&self, log_id: String) -> String {
        let trash_file = trash_dir()
            .join(self.file)
            .try_exists()
            .expect("Cant check whether trash dir exists or not");
        let file_stem = self.file.file_stem().unwrap().to_str().unwrap();
        let file_ext = self.file.extension().and_then(|e| e.to_str());

        let trash_file_name = |stem: &str, ext: Option<&str>| -> String {
            match ext {
                Some(e) => format!("{}.{}.{}", stem, log_id, e),
                None => format!("{}.{}", stem, log_id),
            }
        };

        if !trash_file {
            self.file.file_name().unwrap().to_str().unwrap().to_string()
        } else {
            let trash_name = trash_file_name(file_stem, file_ext);
            self.file
                .with_file_name(trash_name)
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}

pub fn remove_files(items: Vec<PathBuf>, recursive: bool, verbose: bool) -> anyhow::Result<()> {
    if verbose {
        info!("verbose mode on!");
    }
    for item in items {
        if item.exists() {
            let trash = Trash {
                file: item.as_path(),
            };
            trace!("{}", &item.display());
            let id = trash.get_log_id();
            let trash_path = trash_dir().join(trash.trash_name(id.clone()));
            let item_path = current_dir().unwrap().join(&item);
            if verbose {
                trace!("file_path: {}", item_path.display());
            }
            if verbose {
                trace!("trash_path: {}", trash_path.display());
            }
            if recursive {
                if verbose {
                    info!("verbose mode on!");
                }
                info!("recursive remove");
                fs::rename(&item, &trash_path).with_context(|| {
                    format!(
                        "failed to move {} to {}",
                        &item.display(),
                        &trash_path.display()
                    )
                })?;
                write_log(
                    id,
                    item_path.to_str().unwrap().to_string(),
                    trash_path.to_str().unwrap().to_string(),
                )
                .unwrap();
                if verbose {
                    println!("Trashed {} to {}", &item.display(), &trash_path.display());
                }
            } else {
                if verbose {
                    info!("verbose mode on!");
                }
                info!("normal remove");
                if item.is_dir() {
                    eprintln!("{} is a directory.\nTry: rid -r", item.display());
                    return Ok(());
                }
                fs::rename(&item, &trash_path).with_context(|| {
                    format!(
                        "failed to move {} to {}",
                        &item.display(),
                        &trash_path.display()
                    )
                })?;
                write_log(
                    id,
                    item_path.to_str().unwrap().to_string(),
                    trash_path.to_str().unwrap().to_string(),
                )
                .expect("cannot write log");
                if verbose {
                    println!("Trashed {} to {}", &item.display(), &trash_path.display());
                }
            }
        } else {
            eprintln!(
                "rid: cannot remove '{}': no such file or directory",
                &item.display()
            );
            return Ok(());
        }
    }
    Ok(())
}
