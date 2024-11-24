use std::path::Path;

use chrono::{DateTime, Local};
use log::*;

use super::utils::{current_time, trash_dir};

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

impl<'a> TrashOps for Trash<'a> {
    fn exists_in_trash(&self) -> bool
    where
        Self: AsRef<std::path::Path>,
    {
        trash_dir().join(self.file.file_name().unwrap()).exists()
    }
}

impl<'a> Trash<'a> {
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
}
