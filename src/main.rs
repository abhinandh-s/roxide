use clap::Parser;

use std::fs::{self, remove_dir_all};
use std::path::Path;

use self::core::args::{Cli, Commands};
use self::core::history::read_history;
use self::core::rm::core_remove;

pub mod core;

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Some(items) = &cli.file {
        core_remove(items.to_vec(), &cli).unwrap();
        // remove_files(items.to_vec(), &cli).unwrap();
    }

    if let Some(forece_file) = cli.force {
        for item in forece_file {
            if Path::new(&item).exists() {
                if item.is_dir() {
                    fs::remove_dir_all(item).unwrap();
                } else {
                    fs::remove_file(item).unwrap();
                }
            } else {
                println!("Path didnt exists");
            }
        }
    }
    match &cli.command {
        Some(Commands::Revert {}) => {
            read_history().unwrap();
        }
        None => {}
    }
}
