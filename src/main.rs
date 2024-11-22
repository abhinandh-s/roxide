use clap::Parser;

use std::fs::remove_dir_all;
use std::path::Path;

use self::core::args::{Cli, Commands};
use self::revert::read_json_history;
use self::utils_core::remove_files;

pub mod core;

pub mod garbage_collection;
pub mod history;
mod pattern;
pub mod revert;
pub mod utils;
pub mod utils_core;

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Some(items) = &cli.file {
        remove_files(items.to_vec(), &cli).unwrap();
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
