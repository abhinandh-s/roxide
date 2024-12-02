use chrono::{DateTime, Local};
use log::*;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::path::Path;

use crate::verbose;

use super::args::Cli;
use super::helpers::{current_time, trash_dir};

#[derive(Debug)]
pub struct Trash<'a> {
    pub file: &'a Path,
}

pub trait TrashOps {
    fn exists_in_trash(&self) -> bool
    where
        Self: AsRef<std::path::Path>;
    fn get_unique_id(&self) -> String {
        current_time().format("%Y%m%d%H%M%S").to_string()
    }
    fn get_pretty_unique_id(&self) -> String {
        current_time().format("%Y-%m-%d_%H:%M:%S").to_string()
    }
    fn get_raw_unique_id(&self) -> DateTime<Local> {
        current_time()
    }
}

impl TrashOps for Trash<'_> {
    fn exists_in_trash(&self) -> bool
    where
        Self: AsRef<std::path::Path>,
    {
        trash_dir().join(self.file.file_name().unwrap()).exists()
    }
}

impl Trash<'_> {
    pub fn get_log_id(&self) -> (String, String) {
        (
            current_time().format("%Y%m%d%H%M%S").to_string(),
            current_time().format("%Y-%m-%d_%H:%M:%S").to_string(),
        )
    }
    pub fn trash_name(&self, log_id: String) -> String {
        let file_stem = self.file.file_stem().unwrap().to_str().unwrap();
        let file_ext = self.file.extension().and_then(|e| e.to_str());
        let trash_file = trash_dir().join(self.file.file_name().unwrap()).exists();
        let trash_file_name = |stem: &str, ext: Option<&str>| -> String {
            match ext {
                Some(e) => format!("{}.{}.{}", stem, log_id, e),
                None => format!("{}.{}", stem, log_id),
            }
        };
        if !trash_file {
            debug!(
                "impl Trash struct: {:#?}",
                self.file
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap()
            );
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
    /// This function checks the hash of given file and the file in trash directory.
    ///
    /// if hash matches it will return true.
    pub fn compute_sha256(&self, args: &Cli) -> bool {
        if self.file.is_file() {
            let mut file = File::open(self.file).unwrap();
            let trash_file = trash_dir().join(self.file.file_name().unwrap());
            if trash_file.exists() {
                let mut trash_file = File::open(trash_file).unwrap();

                let mut hasher = Sha256::new();
                let mut hasher2 = Sha256::new();

                io::copy(&mut file, &mut hasher).unwrap();
                io::copy(&mut trash_file, &mut hasher2).unwrap();

                let hash = hasher.finalize();
                let hash2 = hasher2.finalize();

                verbose!(
                    args.verbose,
                    "hash of given file and a file in Trash directory matched"
                );
                verbose!(
                    args.verbose,
                    "hash of given {} is: {:x}",
                    self.file.file_name().unwrap().to_string_lossy(),
                    hash
                );
                verbose!(
                    args.verbose,
                    "hash of {} in Trash is: {:x}",
                    self.file.file_name().unwrap().to_string_lossy(),
                    hash2
                );
                if hash == hash2 {
                    return true;
                }
            }
        }
        false
    }
}
