#![allow(unused_labels)]

use std::env::current_dir;
use std::fs::{self, remove_dir, File};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use log::*;

use crate::{
    core::{
        error::Error,
        filter::PathFilter,
        history::{History, LogId, TrashMeta},
        trash::Trash,
        utils::{check_root, trash_dir},
    },
    prompt_yes, show_error, verbose,
};

#[allow(unused_imports)]
use super::args::{Cli, InteractiveMode};

pub type RoError<'a, T> = Result<T, super::error::Error<'a>>;

fn init_checks(item: &Path) -> RoError<()> {
    if item.parent().is_none() && item.has_root() {
        return Err(Error::IsRoot(item));
    }
    Ok(())
}

#[allow(dead_code)]
fn file_write_permission(path: &Path) -> RoError<()> {
    match File::options().read(true).write(true).open(path) {
        Ok(_file) => {}
        Err(_e) => {}
    }

    Ok(())
}

fn init_force_remove(item: &Path) {
    if prompt_yes!("Do you wanna remove it PERMANENTLY?") {
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

fn check_cross_device<'a>(item: &'a Path, trash: &'a Path) -> RoError<'a, ()> {
    let item_metadata = fs::metadata(item).unwrap().dev();
    let trash_metadata = fs::metadata(trash).unwrap().dev();
    // check if the devices are different
    if item_metadata != trash_metadata {
        return Err(Error::CrossesDevices(item));
    }
    Ok(())
}

fn core_remove(args: &Cli, item: &Path) {
    let trash = Trash { file: item };
    let id = trash.get_log_id();
    let item_path = current_dir().unwrap().join(item);
    let trash_path = trash_dir().join(trash.trash_name(id.1));

    // FIX: 2024-11-29
    match check_cross_device(&item_path, &trash_path) {
        Ok(k) => {},
        Err(err) => {}
    }

    if check_root() {
        trace!("is root user");
        show_error!("Can't move item to trash dir while using sudo.");
        init_force_remove(item);
    } else {
        trace!("is normal user");
        let rename_result = fs::rename(
            &item_path,
            trash_dir().join(trash.trash_name(trash.get_log_id().1)),
        );

        if let Err(err) = rename_result {
           /* if let io::ErrorKind::CrossesDevices = err.kind() {
                show_error!(
                    "`{}` is located on a different device. Can't move item to trash dir.",
                    item.display()
                );
                init_force_remove(item);
            } else */ if let io::ErrorKind::PermissionDenied = err.kind() {
                show_error!(
                    "Don't have enough permission to remove `{}`.",
                    item.display()
                );
            } else {
                println!("Error: {}", err);
                init_force_remove(item);
            }
        }

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
}

pub fn init_remove(items: Vec<PathBuf>, args: &Cli) -> anyhow::Result<(), anyhow::Error> {
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
        match result {
            Ok(_) => {}
            Err(_) => {
                eprintln!("{}", Error::DirectoryNotEmpty)
            }
        }
    } else if !path.exists() {
        eprintln!("{}", Error::NoSuchFile(path))
    } else if path.is_file() {
        eprintln!("{}", Error::NotADirectory(path))
    }
}

fn handle_interactive(args: &Cli, item: &Path) {
    // not including InteractiveMode::once and InteractiveMode::Never here
    match args.interactive {
        Some(InteractiveMode::Always) => {
            // for item in items
            // if item.is_symlink() {
            //    println!("got a symlink");
            // }
            if args.dir {
                if prompt_yes!(
                    "do you wanna remove normal empty dir: `{}`?",
                    &item.display()
                ) {
                    remove_empty_dir(item)
                }
            } else if prompt_yes!("do you wanna remove: `{}`?", &item.display()) {
                core_remove(args, item)
            }
        }
        // TODO:
        #[cfg(feature = "extra_commands")]
        Some(InteractiveMode::PromptProtected) => {
            if prompt_yes!("msg") {
                // single_core_remove();
            } else {
                // dont remove
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
    use std::path::Path;
    use std::{fs, path};

    use crate::core::args::Cli;
    use crate::core::rm::check_root;

    use super::init_remove;
    #[test]
    fn test_check_root() {
        // This test assumes a non-root environment.
        let is_root = check_root();
        assert!(!is_root);
    }
    #[test]
    fn revome_files_in_some_dir_from_root() {
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests/revome_files_in_some_dir_from_root");
        fs::create_dir_all(&base_dir).unwrap();
        let files = vec![
            base_dir.join("file1.txt"),
            base_dir.join("file2.pdf"),
            base_dir.join("file3"),
        ];
        for filename in &files {
            fs::write(filename, "some contents").unwrap();
        }

        // no flags test
        let args = Cli {
            file: Some(files.clone()),
            interactive: None,
            recursive: false,
            #[cfg(feature = "extra_commands")]
            check: false,
            force: None,
            list: false,
            verbose: false,
            pattern: None,
            command: None,
            dir: false,
        };

        init_remove(files.clone(), &args).unwrap();
        for filename in &files {
            assert!(!path::Path::new(&filename).exists())
        }
    }
    #[test]
    fn revome_dirs_in_some_dir_from_root() {
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests/revome_dirs_in_some_dir_from_root");
        fs::create_dir_all(&base_dir).unwrap();
        let dirs = vec![
            base_dir.join("dir1"),
            base_dir.join("dir2"),
            base_dir.join("dir3"),
        ];
        for dirnames in &dirs {
            fs::create_dir(dirnames).unwrap();
        }

        // recursive flags test
        let args = Cli {
            file: Some(dirs.clone()),
            interactive: None,
            recursive: true,
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: false,
            force: None,
            list: false,
            verbose: false,
            pattern: None,
            command: None,
        };

        init_remove(dirs.clone(), &args).unwrap();
        for filename in &dirs {
            assert!(!path::Path::new(&filename).exists())
        }
    }
    #[test]
    fn force_revome_dirs_in_some_dir_from_root() {
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests/force_revome_dirs_in_some_dir_from_root");
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir).unwrap();
        }
        let dirs = vec![
            base_dir.join("dir1"),
            base_dir.join("dir2"),
            base_dir.join("dir3"),
        ];
        for dirnames in &dirs {
            if !dirnames.exists() {
                fs::create_dir(dirnames).unwrap();
            }
        }

        // force flags test
        let args = Cli {
            file: None,
            interactive: None,
            recursive: false,
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: false,
            force: Some(dirs.clone()),
            list: false,
            verbose: false,
            pattern: None,
            command: None,
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
    }
    #[test]
    fn list_files_in_some_dir_from_root() {
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests/list_files_in_some_dir_from_root");
        fs::create_dir_all(&base_dir).unwrap();
        let files = vec![
            base_dir.join("file1.txt"),
            base_dir.join("file2.pdf"),
            base_dir.join("file3"),
        ];
        for filename in &files {
            fs::write(filename, "some contents").unwrap();
        }

        // recursive flags test
        let args = Cli {
            file: Some(files.clone()),
            interactive: None,
            recursive: true,
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: false,
            force: None,
            list: true,
            verbose: false,
            pattern: None,
            command: None,
        };

        init_remove(files.clone(), &args).unwrap();
        for filename in &files {
            assert!(path::Path::new(&filename).exists())
        }
    }
    #[test]
    fn pattern_files_in_some_dir_from_root() {
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests/pattern_files_in_some_dir_from_root");
        fs::create_dir_all(&base_dir).unwrap();
        let files = vec![
            base_dir.join("file1.txt"),
            base_dir.join("file2.pdf"),
            base_dir.join("file3"),
        ];
        for filename in &files {
            if !filename.exists() {
                fs::write(filename, "some contents").unwrap();
            }
        }

        // no flags test
        let args = Cli {
            file: Some(files.clone()),
            interactive: None,
            recursive: false,
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: false,
            force: None,
            list: false,
            verbose: false,
            pattern: Some("txt".to_string()),
            command: None,
        };

        init_remove(files.clone(), &args).unwrap();
        assert!(!path::Path::new(&base_dir.join("file1.txt")).exists());
        assert!(path::Path::new(&base_dir.join("file2.pdf")).exists());
        assert!(path::Path::new(&base_dir.join("file3")).exists());
    }
    #[test]
    fn dir_flag_test_001() {
        // propper test empty directory will not fail
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests/dir_flag_test_001");
        fs::create_dir_all(&base_dir).unwrap();
        let dirs = vec![
            base_dir.join("dir1"),
            base_dir.join("dir2"),
            base_dir.join("dir3"),
        ];
        for dirnames in &dirs {
            fs::create_dir(dirnames).unwrap();
        }

        // recursive flags test
        let args = Cli {
            file: Some(dirs.clone()),
            interactive: None,
            recursive: false,
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: true,
            force: None,
            list: false,
            verbose: false,
            pattern: None,
            command: None,
        };

        init_remove(dirs.clone(), &args).unwrap();
        for filename in &dirs {
            assert!(!path::Path::new(&filename).exists())
        }
    }
    #[test]
    fn dir_flag_test_002() {
        // -> Not an empty directory Error
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests/dir_flag_test_002");
        fs::create_dir_all(&base_dir).unwrap();
        let dirs = vec![
            base_dir.join("dir1"),
            base_dir.join("dir2"),
            base_dir.join("dir3"),
        ];
        for dirnames in &dirs {
            if !dirnames.exists() {
                fs::create_dir(dirnames).unwrap();
            }
        }
        let files = vec![
            base_dir.join("dir1/file1.txt"),
            base_dir.join("dir2/file2.pdf"),
            base_dir.join("dir3/file3"),
        ];
        for filename in &files {
            if !filename.exists() {
                fs::write(filename, "some contents").unwrap();
            }
        }
        // recursive flags test
        let args = Cli {
            file: Some(dirs.clone()),
            interactive: None,
            recursive: false,
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: true,
            force: None,
            list: false,
            verbose: false,
            pattern: None,
            command: None,
        };

        init_remove(dirs.clone(), &args).unwrap();
        for filename in &dirs {
            assert!(path::Path::new(&filename).exists())
        }
    }
    #[test]
    fn dir_flag_test_003() {
        // -> Not an empty directory Error
        let base_dir = std::env::current_dir()
            .unwrap()
            .join("trash/tests/dir_flag_test_003");
        fs::create_dir_all(&base_dir).unwrap();
        let dirs = vec![
            base_dir.join("dir1"),
            base_dir.join("dir2"),
            base_dir.join("dir3"),
        ];
        // recursive flags test
        let args = Cli {
            file: Some(dirs.clone()),
            interactive: None,
            recursive: false,
            #[cfg(feature = "extra_commands")]
            check: false,
            dir: true,
            force: None,
            list: false,
            verbose: false,
            pattern: None,
            command: None,
        };

        let result = init_remove(dirs.clone(), &args);
        assert!(result.is_ok());
    }
}
