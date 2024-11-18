use clap::{Parser, Subcommand};
use log::trace;

use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};

use self::core::remove_files;
use self::revert::read_json_history;

pub mod core;
pub mod garbage_collection;
pub mod history;
mod pattern;
pub mod revert;
pub mod utils;

#[derive(Parser)]
#[command(
    version,
    name = "roxide",
    author = "Abhinandh S <ugabhi@proton.me>",
    about = "roxide",
    //long_about = "By default, roxide does not remove directories.Use the --recursive (-r) option to remove each listed directory, too, along with all of its contents.\n
    //    To remove a file whose name starts with a '-', for example '-foo',\n
    //    use one of these commands:\n
    //    roxide -- -foo\n
    //    roxide ./-foo\n
    //    If you use roxide to remove a file, it might be possible to recover the file/directory.\n
    //    Files are trashed to XDG specified trash directory.\n
    //    Example:\n
    //    `$HOME`/.local/share/Trash/files\n"
)]
struct Cli {
    /// Remove files
    file: Option<Vec<PathBuf>>,

    /// Remove directories and their contents recursively
    #[arg(short, long)]
    recursive: bool,

    /// remove files matching the pattern. revert will not work on patterns, provide -rp for recursive remove
    #[arg(short = 'p', long = "pattern", value_name = "PATTERN")] // roxide some/dir -p .pdf
    pattern: Option<String>, // Accept the file-matching pattern

    /// Forces deletion without moving files to the trash directory
    #[arg(short, long, value_name = "FILE")]
    force: Option<Vec<PathBuf>>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// revert the previous remove
    Revert {},
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Some(items) = cli.file {
        trace!("got item:{:?} and recursive is {:?}", &items, cli.recursive);
        remove_files(items, cli.recursive, cli.pattern, cli.verbose).unwrap();
    }

    if let Some(forece_file) = cli.force {
        for i in forece_file {
            if Path::new(&i).exists() {
                remove_dir_all(i).unwrap();
            } else {
                println!("Path didnt exists");
            }
        }
    }
    match &cli.command {
        Some(Commands::Revert {}) => {
            read_json_history().unwrap();
        }
        None => {}
    }
}
