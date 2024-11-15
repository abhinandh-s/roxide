use clap::{Parser, Subcommand};
use log::trace;

use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};

use self::core::remove_files;
use self::revert::read_json_history;
use self::pattern::pattern;

pub mod core;
pub mod garbage_collection;
pub mod history;
pub mod revert;
pub mod utils;
pub mod pattern;

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

    /// recursive remove files matching the pattern, path must be provided with -d flag 
    #[arg(short = 'p', long = "pattern")]
    pattern: Option<String>, // Accept the file-matching pattern

    /// Path for pattern flag
    #[arg(short = 'd', long = "path")]
    path: Option<PathBuf>, // Accept the directory path

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
    Revert {
        //    num: i8,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Some(items) = cli.file {
        trace!("{:#?}", &items);
        remove_files(items, cli.recursive, cli.verbose, true).unwrap();
    }


    if let (Some(pattern_str), Some(path)) = (cli.pattern, cli.path) {
        trace!("Pattern: {:#?}, Path: {:#?}", pattern_str, path);
        pattern(path.to_str().unwrap().to_string(), pattern_str).unwrap(); 
    } else {
        eprintln!("For pattern to work path must be provided with -d flag.\nEg: roxide -p .org -d directory/");
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
        } // Some(Commands::Revert {  }) => read_history(), // write_log(num).unwrap(),
        None => {}
    }
}
