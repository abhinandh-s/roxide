use clap::Parser;

use std::fs;
use std::path::Path;

use self::core::args::{Cli, Commands};
use self::core::check::checks_init;
use self::core::config::read_config;
use self::core::history::read_history;
use self::core::rm::init_remove;

pub mod core;

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Some(items) = &cli.file {
        init_remove(items.to_vec(), &cli).unwrap();
        // remove_files(items.to_vec(), &cli).unwrap();
    }
    
    #[cfg(feature = "extra_commands")]
    if cli.dev {
        read_config("config.toml").unwrap();
    }
    #[cfg(feature = "extra_commands")]
    if cli.check {
        checks_init();
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
