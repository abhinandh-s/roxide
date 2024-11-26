#![allow(unused_labels)]

use std::env::current_dir;
use std::fs::{self, remove_dir, File};
use std::path::{Path, PathBuf};

use log::*;

use crate::core::error::Error;
use crate::core::filter::filter_paths;
use crate::core::history::write_history;
use crate::core::trash::Trash;
use crate::core::utils::{check_root, trash_dir};
use crate::{prompt_yes, show_error, verbose};

#[allow(unused_imports)]
use super::args::{Cli, InteractiveMode};

pub type RoError<'a, T> = Result<T, super::error::Error<'a>>;

fn init_checks(item: &Path) -> RoError<()> {
    if item.parent().is_none() && item.has_root() {
        return Err(Error::IsRoot(item));
    }
    Ok(())
}

pub fn file_write_permission(path: &Path) -> RoError<()> {
    match File::options().read(true).write(true).open(path) {
        Ok(_file) => {}
        Err(_e) => {}
    }

    Ok(())
}

pub fn normal_remove(_args: &Cli) {
    show_error!("sudo support is work in progress");
}

fn core_remove(args: &Cli, item: &Path) {
    let trash = Trash { file: item };
    let id = trash.get_log_id();
    trace!("id: {:?}", id);
    let item_path = current_dir().unwrap().join(item);
    trace!("item path recursive true: {:?}", item_path);
    let trash_path = trash_dir().join(trash.trash_name(id.1));
    trace!("trash path recursive true: {:?}", trash_path);
    if check_root() {
        trace!("is root user");
        if prompt_yes!(
            "Can't move item to trash dir while using sudo. Do you wanna remove {}: {} permanently?",
            if item.is_file() || item.is_symlink() {
                "file"
            } else {
                "dir"
            },
            item.display()
        ) {
            normal_remove(args)
        }
    } else {
        trace!("is normal user");
        let rename_result = fs::rename(
            &item_path,
            trash_dir().join(trash.trash_name(trash.get_log_id().1)),
        );

        if let Err(e) = rename_result {
            show_error!("Error moving {}: {:?}", item.display(), e);
            if prompt_yes!(
                "can't remove {} to trash dir. Do you wanna remove it?",
                item.display()
            ) {
                if item.is_file() {
                    fs::remove_file(item).unwrap();
                } else {
                    // Removes a directory at this path, after removing all its contents.
                    // This function does **not** follow symbolic links and
                    // it will simply remove the symbolic link itself.
                    fs::remove_dir_all(item).unwrap();
                }
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
            write_history(
                id.0,
                item_path.to_str().unwrap().to_string(),
                trash_path.to_str().unwrap().to_string(),
            )
            .unwrap();
        }
    }
}

pub fn init_remove(items: Vec<PathBuf>, args: &Cli) -> anyhow::Result<(), anyhow::Error> {
    let entries = match filter_paths(items, args) {
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
            #[cfg(feature = "extra_commands")]
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
            #[cfg(feature = "extra_commands")]
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
            #[cfg(feature = "extra_commands")]
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
            #[cfg(feature = "extra_commands")]
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
            #[cfg(feature = "extra_commands")]
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
}
