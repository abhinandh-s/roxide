use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::io::{self, Result};

include!("src/core/args.rs");

fn main() -> io::Result<()> {
    // Tell Cargo to rerun this script if `build.rs` changes
    // Since we are generating completions in the package directory, we need to
    // set this so that Cargo doesn't rebuild every time.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:rerun-if-changed=tests/");
    generate_completions()
}

fn generate_completions() -> Result<()> {
    const BIN_NAME: &str = "roxide";
    const OUT_DIR: &str = "completions";

    let mut cmd = Cli::command();
    
    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Elvish] {
        generate_to(shell, &mut cmd, BIN_NAME, OUT_DIR)?;
    }

    Ok(())
}
