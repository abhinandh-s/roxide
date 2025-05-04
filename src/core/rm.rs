#![allow(unused_labels, unused_imports)]

use std::env::current_dir;
use std::fs::{self, remove_dir, File};
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use log::*;
use roxide::{prompt_yes, show_error, verbose};

use crate::core::checks::check_cross_device;
use crate::core::{
    filter::PathFilter,
    helpers::trash_dir,
    history::{History, LogId, TrashMeta},
    trash::Trash,
};
use crate::utils::config::init_config;

use roxide::RoxError as Error;

use super::args::{Cli, InteractiveMode};
use super::checks::check_root;

pub type RoError<'a, T> = Result<T, Error>;

fn init_checks(item: &Path) -> RoError<()> {
    if item.parent().is_none() && item.has_root() {
        return Err(Error::IsRoot(item.to_path_buf()));
    }
    Ok(())
}

fn init_force_remove_with_prompt(item: &Path) {
    if prompt_yes!("remove it PERMANENTLY?") {
        if item.is_file() {
            if let Err(e) = fs::remove_file(item) {
                show_error!("Failed to remove file: {}", e);
            }
        } else if item.is_dir() {
            if let Err(e) = fs::remove_dir_all(item) {
                show_error!("Failed to remove directory: {}", e);
            }
        }
    }
}

fn init_force_remove_without_prompt(item: &Path) {
    if item.is_file() {
        if let Err(e) = fs::remove_file(item) {
            show_error!("Failed to remove file: {}", e);
        }
    } else if item.is_dir() {
        if let Err(e) = fs::remove_dir_all(item) {
            show_error!("Failed to remove directory: {}", e);
        }
    }
}

fn core_remove(args: &Cli, item: &Path) {
    let trash = Trash { file: item };
    let id = trash.get_log_id();
    let item_path = current_dir().unwrap().join(item);
    let trash_path = trash_dir().join(trash.trash_name(id.1));

    let config = init_config();

    if check_root() {
        trace!("is root user");
        show_error!("Can't move item to trash dir while using sudo.");
        init_force_remove_with_prompt(item);
    } else {
        trace!("is normal user");

        // we can't move items from an another device.
        // only option is to copy or delete
        // So. we will prompt for force remove
        match check_cross_device(&item_path) {
            Ok(()) => match config.settings.check_sha256 {
                Some(true) if trash.compute_sha256(args) && item.is_file() => {
                    init_force_remove_without_prompt(&item_path);
                    verbose!(
                        args.verbose,
                        "roxide: removed {} permanently",
                        &item_path.display()
                    );
                }
                _ => {
                    let rename_result = fs::rename(
                        &item_path,
                        trash_dir().join(trash.trash_name(trash.get_log_id().1)),
                    );
                    match rename_result {
                        Ok(_) => {
                            if args.pattern.is_none() {
                                verbose!(
                                    args.verbose,
                                    "Trashed {} to {}",
                                    item.display(),
                                    trash_dir()
                                        .join(trash.trash_name(trash.get_log_id().1))
                                        .display()
                                );
                                let history = History {
                                    log_id: LogId::from_str(id.0.to_string().as_str()).unwrap(),
                                    metadata: TrashMeta {
                                        file_path: item_path,
                                        trash_path,
                                    },
                                };
                                History::write(history).unwrap();
                            }
                        }
                        Err(err) => match err.kind() {
                            io::ErrorKind::PermissionDenied => {
                                show_error!(
                                    "Don't have enough permission to remove `{}`.",
                                    item.display()
                                );
                            }
                            // NOTE: unstable
                            // io::ErrorKind::ResourceBusy => {
                            //     show_error!(
                            //         "Resource is busy and cannot be moved: {}",
                            //         item.display()
                            //     );
                            // }
                            // io::ErrorKind::ReadOnlyFilesystem => {
                            //     show_error!(
                            //         "can't move. error: ReadOnly Filesystem: {}",
                            //         item.display()
                            //     );
                            //     init_force_remove_with_prompt(item);
                            // }
                            _ => {
                                println!("Error: {}", err);
                                init_force_remove_with_prompt(item);
                            }
                        },
                    }
                }
            },
            Err(err) => {
                show_error!("{}", err);
                init_force_remove_with_prompt(item);
            }
        }
    }
}

pub fn init_remove(items: Vec<PathBuf>, args: &Cli) -> RoError<()> {
    let entries = match PathFilter::init(items, args) {
        Ok(filtered) => filtered,
        Err(e) => {
            eprintln!("{}", e);
            Vec::with_capacity(0)
        }
    };
    handle_interactive_once(args);
    for item in &entries {
        if args.list {
            println!("{}", item.display());
        } else if let Err(e) = init_checks(item) {
            eprintln!("Error: {}", e); // prints Error::IsRoot
            continue;
        } else {
            handle_interactive(args, item)
        }
    }
    trace!("{:#?}", entries);
    Ok(())
}

fn handle_interactive_once(args: &Cli) -> bool {
    let items = args.file.as_ref().unwrap();
    if args.interactive == Some(InteractiveMode::Once) && (items.len() > 3 || args.recursive) {
        let msg: String = format!(
            "remove {} {}{}",
            items.len(),
            if items.len() > 1 {
                "arguments"
            } else {
                "argument"
            },
            if args.recursive { " recursively?" } else { "?" }
        );
        if prompt_yes!("{}", msg) {
            return true;
        }
    }
    false
}

fn remove_empty_dir(path: &Path) {
    if path.exists() && path.is_dir() {
        let result = remove_dir(path);
        if let Err(result) = result {
            eprintln!(
                "{}",
                roxide::RoxError::DirectoryNotEmpty(result.to_string())
            )
        }
    } else if !path.exists() {
        eprintln!("{}", Error::NoSuchFile(path.to_path_buf()))
    } else if path.is_file() {
        eprintln!("{}", Error::NotADirectory(path.to_path_buf()))
    }
}

fn handle_interactive(args: &Cli, item: &Path) {
    // File::open(path) doesn't open the file in write mode
    // So, we need to use file options to open it in write mode to check if we have write permission
    #[cfg(feature = "extra_commands")]
    let file_write_permission = File::options().read(true).write(true).open(item).is_ok();
    // not including InteractiveMode::once and InteractiveMode::Never here
    match args.interactive {
        Some(InteractiveMode::Always) => {
            if args.dir {
                if prompt_yes!("remove normal empty dir: `{}`?", &item.display()) {
                    remove_empty_dir(item)
                }
            } else if prompt_yes!("remove: `{}`?", &item.display()) {
                core_remove(args, item)
            }
        }
        #[cfg(feature = "extra_commands")]
        Some(InteractiveMode::PromptProtected) => {
            if !file_write_permission
                && prompt_yes!("write protected file, remove: `{}`?", &item.display())
            {
                core_remove(args, item);
            }
        }
        Some(_) => {
            if args.dir {
                remove_empty_dir(item)
            } else {
                core_remove(args, item)
            }
        }
        None => {
            if args.dir {
                remove_empty_dir(item)
            } else {
                core_remove(args, item)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use std::fs::{remove_dir_all, File};
    use std::path::{Path, PathBuf};
    use std::thread::sleep;
    use std::time::Duration;
    use std::{fs, path};

    use dirs::data_dir;
    use tempdir::TempDir;

    use crate::core::args::Cli;
    use crate::core::helpers::trash_dir;
    use crate::core::rm::check_root;
    use crate::utils::config::init_config;

    use super::init_remove;

    /// will create a empty dir2 and a dir1 with 3 files
    fn make_dirs_for_test(basedir_name: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests")
            .join(basedir_name);
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir).unwrap();
        }
        let dirs = vec![base_dir.join("dir1"), base_dir.join("dir2")];
        let dirs_cow = Cow::Borrowed(&dirs);
        for dirs in dirs_cow.iter() {
            if !base_dir.join(dirs).exists() {
                fs::create_dir(base_dir.join(dirs)).unwrap();
            }
        }
        let files = vec![
            base_dir.join("dir1/file1.txt"),
            base_dir.join("dir1/file2.pdf"),
            base_dir.join("dir1/file3"),
        ];
        let files_cow = Cow::Borrowed(&files);
        for filename in files_cow.iter() {
            if !filename.exists() {
                fs::write(filename, "some contents").unwrap();
            }
        }
        for dirname in dirs_cow.iter() {
            assert!(path::Path::new(&dirname).exists())
        }
        for filename in &files {
            assert!(path::Path::new(&filename).exists())
        }
        (dirs_cow.to_vec(), files_cow.to_vec())
    }
    fn remove_test_dir(basedir_name: &Path) {
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests")
            .join(basedir_name);
        remove_dir_all(&base_dir).unwrap();
        assert!(!path::Path::new(&base_dir).exists())
    }

    #[test]
    fn test_check_root() {
        // This test assumes a non-root environment.
        let is_root = check_root();
        assert!(!is_root);
    }

    #[test]
    // #[ignore = "i have no idea why this is failing. It works on manual tests"]
    fn check_sha256_01() {
        let items = make_dirs_for_test(Path::new("check_sha256_01"));
        let dirs = items.0;
        let files = items.1;
        let _files_cow = Cow::Borrowed(&files);
        let dirs_cow = Cow::Borrowed(&dirs);

        let args = Cli {
            file: Some(dirs_cow.to_vec()),
            interactive: None,
            recursive: true,
            #[cfg(feature = "extra_commands")]
            check: false,
            ..Default::default()
        };
        // panic!("{:#?}", dirs_cow);
        sleep(Duration::from_secs(1));
        init_remove(dirs_cow.to_vec(), &args).unwrap();
        for filename in &files {
            assert!(!path::Path::new(&filename).exists())
        }
        for dirname in dirs_cow.iter() {
            assert!(!path::Path::new(&dirname).exists())
        }
        remove_test_dir(Path::new("check_sha256_01"));
    }

    /// no flag test
    /// only files as input no dirs
    #[test]
    fn revome_files_only_01() {
        let items = make_dirs_for_test(Path::new("revome_files_only_01"));
        let _dirs = items.0;
        let files = items.1;

        let args = Cli {
            file: Some(files.to_vec()),
            #[cfg(feature = "extra_commands")]
            check: false,
            ..Default::default()
        };

        sleep(Duration::from_secs(1));
        init_remove(files.clone(), &args).unwrap();
        for filename in &files {
            assert!(!path::Path::new(&filename).exists())
        }
        remove_test_dir(Path::new("revome_files_only_01"));
    }
    /// recursive flags test
    /// revome_dirs_in_some_dir_from_root
    #[test]
    fn recursive_remove_01() {
        let items = make_dirs_for_test(Path::new("recursive_remove_01"));
        let dirs = items.0;
        let files = items.1;
        let dirs_cow = Cow::Borrowed(&dirs);

        let args = Cli {
            file: Some(dirs_cow.to_vec()),
            recursive: true,
            #[cfg(feature = "extra_commands")]
            check: false,
            ..Default::default()
        };

        sleep(Duration::from_secs(1));
        init_remove(dirs_cow.to_vec(), &args).unwrap();

        for filename in files {
            assert!(!path::Path::new(&filename).exists())
        }
        // remove_test_dir(Path::new("recursive_remove_01"));
    }
    #[test]
    fn force_revome_dirs() {
        let items = make_dirs_for_test(Path::new("force_revome_dirs"));
        let dirs = items.0;
        let _files = items.1;
        let dirs_cow = Cow::Borrowed(&dirs);

        // force flags test
        let args = Cli {
            #[cfg(feature = "extra_commands")]
            check: false,
            force: Some(dirs_cow.to_vec()),
            ..Default::default()
        };

        if let Some(forece_file) = args.force {
            for item in forece_file {
                if Path::new(&item).exists() {
                    if item.is_dir() {
                        fs::remove_dir_all(item).expect("Error while removing dirs");
                    } else {
                        fs::remove_file(item).expect("Error while removing files");
                    }
                } else {
                    println!("Path didnt exists");
                }
            }
        }
        for filename in &dirs {
            assert!(!path::Path::new(&filename).exists())
        }
        remove_test_dir(Path::new("force_revome_dirs"));
    }
    #[test]
    fn list_files_flag() {
        let items = make_dirs_for_test(Path::new("list_files_flag"));
        let dirs = items.0;
        let files = items.1;
        let files_cow = Cow::Borrowed(&files);

        let args = Cli {
            file: Some(files_cow.to_vec()),
            recursive: true,
            #[cfg(feature = "extra_commands")]
            check: false,
            list: true,
            ..Default::default()
        };

        sleep(Duration::from_secs(1));
        init_remove(files_cow.to_vec(), &args).unwrap();
        for filename in &files {
            assert!(path::Path::new(&filename).exists())
        }
        for dirname in &dirs {
            assert!(path::Path::new(&dirname).exists())
        }
        remove_test_dir(Path::new("list_files_flag"));
    }
    #[test]
    fn pattern_flag() {
        let items = make_dirs_for_test(Path::new("pattern_flag"));
        let _dirs = items.0;
        let files = items.1;
        let files_cow = Cow::Borrowed(&files);

        // no flags test
        let args = Cli {
            file: Some(files_cow.to_vec()),
            #[cfg(feature = "extra_commands")]
            check: false,
            pattern: Some("txt".to_string()),
            ..Default::default()
        };

        sleep(Duration::from_secs(1));
        init_remove(files_cow.to_vec(), &args).unwrap();
        let f = files.clone();
        assert!(!path::Path::new(&f[0]).exists()); // this one matches the pattern
        assert!(path::Path::new(&f[1]).exists());
        assert!(path::Path::new(&f[2]).exists());
        remove_test_dir(Path::new("pattern_flag"));
    }
    #[test]
    fn dir_flag_01() {
        let items = make_dirs_for_test(Path::new("dir_flag_01"));
        let dirs = items.0;
        let files = items.1;
        let _files_cow = Cow::Borrowed(&files);
        let dirs_cow = Cow::Borrowed(&dirs);

        // recursive flags test
        let args = Cli {
            file: Some(dirs_cow.to_vec()),
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: true,
            ..Default::default()
        };

        sleep(Duration::from_secs(1));
        init_remove(dirs_cow.to_vec(), &args).unwrap();

        let d = dirs.clone();
        assert!(path::Path::new(&d[0]).exists());
        assert!(!path::Path::new(&d[1]).exists()); // this one is the empty one
        remove_test_dir(Path::new("dir_flag_01"));
    }
    #[test]
    fn dir_flag_02() {
        // -> Not an empty directory Error
        let items = make_dirs_for_test(Path::new("dir_flag_02"));
        let dirs = items.0;
        let files = items.1;
        let _files_cow = Cow::Borrowed(&files);
        let dirs_cow = Cow::Borrowed(&dirs);

        let args = Cli {
            file: Some(dirs_cow.to_vec()),
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: true,
            ..Default::default()
        };

        sleep(Duration::from_secs(1));
        init_remove(dirs_cow.to_vec(), &args).unwrap();

        let d = dirs.clone();
        assert!(path::Path::new(&d[0]).exists());
        assert!(!path::Path::new(&d[1]).exists()); // this one is the empty one
        remove_test_dir(Path::new("dir_flag_02"));
    }
    #[test]
    fn dir_flag_03() {
        // -> Not an empty directory Error
        let items = make_dirs_for_test(Path::new("dir_flag_03"));
        let dirs = items.0;
        let files = items.1;
        let _files_cow = Cow::Borrowed(&files);
        let dirs_cow = Cow::Borrowed(&dirs);

        let args = Cli {
            file: Some(dirs_cow.to_vec()),
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: true,
            ..Default::default()
        };
        sleep(Duration::from_secs(1));
        let result = init_remove(dirs_cow.to_vec(), &args);
        assert!(result.is_ok());
        let d = dirs.clone();
        assert!(path::Path::new(&d[0]).exists());
        assert!(!path::Path::new(&d[1]).exists()); // this one is the empty one
        remove_test_dir(Path::new("dir_flag_03"));
    }
}
