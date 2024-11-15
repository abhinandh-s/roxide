use clap::{Parser, Subcommand};
use log::trace;

use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};

use self::core::remove_files;
use self::garbage_collection::gc;
use self::history::write_history;
use self::revert::read_json_history;
use self::wild_cards::wild_card;

pub mod core;
pub mod garbage_collection;
pub mod history;
pub mod revert;
pub mod utils;
pub mod wild_cards;

#[derive(Parser)]
#[command(
    version,
    name = "rid",
    author = "Abhinandh S <ugabhi@proton.me>",
    about = "rid",
    //long_about = "By default, rid does not remove directories.Use the --recursive (-r) option to remove each listed directory, too, along with all of its contents.\n
    //    To remove a file whose name starts with a '-', for example '-foo',\n
    //    use one of these commands:\n
    //    rid -- -foo\n
    //    rid ./-foo\n
    //    If you use rid to remove a file, it might be possible to recover the file/directory.\n
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

    /// Remove directories and their contents recursively
    #[arg(short, long, value_name = "FILE")]
    force: Option<Vec<PathBuf>>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// For testing porpose wont work
    #[arg(short, long)]
    json: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Turn debugging information on
    Debug {
        #[arg(short, long)]
        list: bool,
    },
    /// alpha stage
    Gc {
        // #[arg(short, long)]
        date: i8,
    },
    /// revert the previous remove
    Revert {
        //    num: i8,
    },
    /// recursive remove files with wild card and patterns
    WildCard {
        // #[arg(short, long)]
        pattern: String,
        path: String
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Some(items) = cli.file {
        trace!("{:#?}", &items);
        remove_files(items, cli.recursive, cli.verbose).unwrap();
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

    if let Some(t) = cli.json {
        if t {
            write_history().unwrap();
        } else {
            println!("try true");
        }
    }

    match &cli.command {
        Some(Commands::Debug { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Gc { date }) => gc(date).unwrap(),
        Some(Commands::Revert {}) => {
            read_json_history().unwrap();
        } // Some(Commands::Revert {  }) => read_history(), // write_log(num).unwrap(),
        Some(Commands::WildCard { pattern, path }) => wild_card(path.to_string(), pattern.to_string()).unwrap(),
        None => {}
    }
}
