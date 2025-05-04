use clap::{Parser, Subcommand, ValueEnum};

use std::path::PathBuf;

#[derive(Parser, Default, Debug)]
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
pub struct Cli {
    /// Remove files
    pub file: Option<Vec<PathBuf>>,

    /// Remove directories and their contents recursively
    #[arg(short, long)]
    pub recursive: bool,

    /// list items which will be affected, (dry run)
    #[arg(short, long)]
    pub list: bool,

    /// whether to prompt before removals
    #[arg(short, long)]
    pub interactive: Option<InteractiveMode>,

    /// remove files matching the pattern. revert will not work on patterns, provide -rp for recursive remove
    #[arg(short = 'p', long = "pattern", value_name = "PATTERN")] // roxide some/dir -p .pdf
    pub pattern: Option<String>, // Accept the file-matching pattern

    /// Forces deletion without moving files to the trash directory
    #[arg(short, long, value_name = "FILE")]
    pub force: Option<Vec<PathBuf>>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// remove empty directories    
    #[arg(short, long)]
    pub dir: bool,
    // given item is not a directory
    // item doesn't exists
    // directory is not empty
    #[cfg(feature = "extra_commands")]
    /// Will check health of roxide in user env
    #[arg(short, long)]
    pub check: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// revert the previous remove
    Revert,
}

/// Enum, determining when the `rm` will prompt the user about the file deletion
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default, Debug)]
pub enum InteractiveMode {
    /// Never prompt
    #[default]
    Never,
    /// Prompt once before removing more than three files
    /// or when removing recursivly.
    Once,
    /// Prompt before every removal
    Always,
    #[cfg(feature = "extra_commands")]
    /// Prompt only on write-protected files
    PromptProtected,
}
