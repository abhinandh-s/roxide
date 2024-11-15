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

pub fn remove_files(items: Vec<PathBuf>, recursive: bool, verbose: bool, log: bool) -> anyhow::Result<()> {
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
                if log {
                    write_log(
                        id,
                        item_path.to_str().unwrap().to_string(),
                        trash_path.to_str().unwrap().to_string(),
                    )
                        .unwrap();
                }
                if verbose {
                    println!("Trashed {} to {}", &item.display(), &trash_path.display());
                }
            } else {
                if verbose {
                    info!("verbose mode on!");
                }
                info!("normal remove");
                if item.is_dir() {
                    eprintln!("{} is a directory.\nTry: roxide -r", item.display());
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
                "roxide: cannot remove '{}': no such file or directory",
                &item.display()
            );
            return Ok(());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;
    use std::path::{Path, PathBuf};
    use std::{fs, path};

    use super::remove_files;

    #[test]
    fn glob_test() {
        // FIX: need fix, not an good test
        let files_for_glob_test = vec![
            PathBuf::from("glob_test_file_001.txt"),
            PathBuf::from("glob_test_file_002.txt"),
            PathBuf::from("glob_test_file_003.txt"),
            PathBuf::from("glob_test_file_004.txt"),
            PathBuf::from("glob_test_file_005.txt"),
            PathBuf::from("glob_test_file_006.txt"),
            PathBuf::from("glob_test_file_007.txt"),
            PathBuf::from("glob_test_file_008.txt"),
        ];
        for i in &files_for_glob_test {
            println!("{}", i.display());
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file glob_test_file_00x.txt"));
        }
        remove_files(files_for_glob_test.clone(), false, true, true).unwrap();
        for i in files_for_glob_test {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }

    #[test]
    fn single_file_test() {
        let v0 = PathBuf::from("temp_file_for_single_file_text01.txt");
        let v1 = PathBuf::from("temp_file_for_single_file_text02.txt");
        let v2 = PathBuf::from("temp_file_for_single_file_text03.txt");
        let v3 = PathBuf::from("temp_file_for_single_file_text04.txt");
        let single_files = vec![v0, v1, v2, v3];
        for i in &single_files {
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
        remove_files(single_files.clone(), false, true, true).unwrap();
        for i in &single_files {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }
    #[test]
    fn single_hidden_file_test() {
        let v0 = PathBuf::from(".tmp_hidden_file_for_single_file_text01.txt");
        let v1 = PathBuf::from(".tmp_hidden_file_for_single_file_text02.txt");
        let v2 = PathBuf::from(".tmp_hidden_file_for_single_file_text03.txt");
        let v3 = PathBuf::from(".tmp_hidden_file_for_single_file_text04.txt");
        let single_files = vec![v0, v1, v2, v3];
        for i in &single_files {
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
            pretty_assertions::assert_eq!(1, 1);
        }
        remove_files(single_files.clone(), false, true, true).unwrap();
        for i in &single_files {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }
    #[test]
    fn remove_file_from_dir_test() {
        let s = Path::new("some_other").exists();
        if s {
            remove_dir_all("some_other").unwrap();
        }
        fs::create_dir_all("some_other/dir").unwrap();
        fs::write("some_other/dir/test.txt", "some contents for testing").unwrap();
        let v3 = PathBuf::from("some_other/dir/test.txt");
        let single_files = vec![v3];
        remove_files(single_files, false, true, true).expect("Err with my function");
        assert!(!fs::exists("some_other/dir/test.txt")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        remove_files(
            vec![path::Path::new("some_other").to_path_buf()],
            true,
            true,
            true
        )
        .expect("Err with my function");
        assert!(!fs::exists("some_other")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
    // TEST: done
    #[test]
    fn remove_file_from_hidden_dir_test() {
        let s = Path::new(".some_hidden").exists();
        if s {
            remove_dir_all(".some_hidden").expect("Failed to remove existing .some_hidden dir");
        } else {
            // creates a hidden dir and a file
            fs::create_dir_all(".some_hidden/dir").expect("Failed to create .some_hidden/dir for remove_file_from_hidden_dir_test test");
            fs::write(".some_hidden/dir/test.txt", "some contents for testing").unwrap();
            let v3 = PathBuf::from(".some_hidden/dir/test.txt");
            let a_single_file_from_hidden_dir = vec![v3];
            remove_files(a_single_file_from_hidden_dir, false, true,true).expect("Err with my function remove_file_from_hidden_dir_test");
            assert!(!fs::exists(".some_hidden/dir/test.txt").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
            remove_files(
                vec![path::Path::new(".some_hidden").to_path_buf()],
                true,
                true,
                true,
            )
                .expect("Err with my function remove_files in remove_file_from_hidden_dir_test");
            assert!(!fs::exists(".some_hidden")
                .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        }
    }
    #[test]
    fn recursive_remove_test() {
        let s = Path::new("some").exists();
        if s {
            remove_dir_all("some").unwrap();
        }
        fs::create_dir_all("some/dir/for/testing").unwrap();
        let test_dir = vec![PathBuf::from("some")];
        fs::write("some/test.txt", "some contents for testing").unwrap();
        remove_files(test_dir, true, true, true).expect("Err with my function");
        assert!(!fs::exists("some")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for/testing")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
    #[test]
    fn recursive_hidden_dir_remove_test() {
        let s = Path::new(".some").exists();
        if s {
            remove_dir_all(".some").unwrap();
        }
        fs::create_dir_all(".some/dir/for/testing").unwrap();
        let test_dir = vec![PathBuf::from(".some")];
        fs::write(".some/test.txt", "some contents for testing").unwrap();
        remove_files(test_dir, true, true, true).expect("Err with my function");
        assert!(!fs::exists(".some")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists(".some/dir/for")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists(".some/dir/for/testing")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
}
