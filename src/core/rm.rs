#![allow(unused_labels)]

use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use log::*;

use crate::core::filter::filter_paths;
use crate::core::history::write_history;
use crate::core::trash::Trash;
use crate::core::utils::trash_dir;
use crate::{prompt_yes, show_error, verbose};

#[allow(unused_imports)]
use super::args::{Cli, InteractiveMode};

// Checks if the current user is root.
/// Returns `true` if the user is root, otherwise `false`.
pub fn check_root() -> bool {
    // Safe implementation: Read UID from `/proc/self/status`
    use std::fs;
    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("Uid:") {
                let uid = line.split_whitespace().nth(1);
                return uid == Some("0");
            }
        }
    }
    false
}

/// TODO:
/// rm --interactive=always trash/ -r
/// rm: descend into directory 'trash/'? y
/// rm: descend into directory 'trash/one'? y
/// rm: remove regular empty file 'trash/one/two.txt'? n
/// rm: remove directory 'trash/one'? n
/// rm: remove directory 'trash/'? n
pub fn handle_interactive_once(args: &Cli) -> bool {
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
        // prompt_yes!("there is no trash dir for sudo user, do you wanna remove the file?")
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
                if item.is_dir() {
                    // Removes a directory at this path, after removing all its contents.
                    // This function does **not** follow symbolic links and
                    // it will simply remove the symbolic link itself.
                    fs::remove_dir_all(item).unwrap();
                } else {
                    fs::remove_file(item).unwrap();
                }
            }
        }
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

pub fn init_remove(items: Vec<PathBuf>, args: &Cli) -> anyhow::Result<(), anyhow::Error> {
    let enties = filter_paths(items, args).unwrap();
    handle_interactive_once(args);
    for item in &enties {
        if args.list {
            println!("{}", item.display());
        } else {
            let is_root = item.parent().is_none() && item.has_root();
            if is_root {
                show_error!("{} is root!", item.display());
                continue;
            } else {
                handle_interactive(args, item)
            }
        }
    }
    trace!("{:#?}", enties);
    Ok(())
}

fn handle_interactive(args: &Cli, item: &Path) {
    // not including InteractiveMode::once and InteractiveMode::Never here
    match args.interactive {
        Some(InteractiveMode::Always) => {
            // for item in items
            if prompt_yes!("do you wanna remove: `{}`?", &item.display()) {
                core_remove(args, item)
            }
        }
        // TODO:
        #[cfg(feature = "wip")]
        Some(InteractiveMode::PromptProtected) => {
            if prompt_yes!("msg") {
                // single_core_remove();
            } else {
                // dont remove
            }
        }
        Some(_) => core_remove(args, item),
        None => {}
    }
}
