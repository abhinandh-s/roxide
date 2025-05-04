use clap::Parser;

use std::{fs, path::Path};

use self::core::{
    args::{Cli, Commands},
    history::History,
    rm::init_remove,
};

pub mod core;
pub mod utils;

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Some(items) = &cli.file {
        init_remove(items.to_vec(), &cli).unwrap();
    }

    if let Some(forece_file) = cli.force {
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
    match &cli.command {
        Some(Commands::Revert) => {
            History::init_revert().unwrap();
        }
        None => {}
    }
}
