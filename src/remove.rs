use std::path::PathBuf;

use derive_builder::Builder;

pub static HELP_FOOTER: &str = include_str!("../docs/help_footer.md");

#[derive(Debug, Clone, Default, Builder)]
#[builder(setter(into))]
pub struct Remover {
    pub paths: Vec<PathBuf>,
    pub force: bool,
    pub recursive: bool,
    pub dir: bool,
    pub verbose: bool,
    pub intractive: Intractive, // prompt before every removal
    pub one_file_system: bool,
    pub no_preserve_root: bool,
    pub preserve_root: bool,
}

impl Remover {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_paths(&mut self, paths: Vec<PathBuf>) {
        self.paths = paths;
    }

    pub fn set_force(&mut self, force: bool) {
        self.force = force;
    }

    pub fn set_recursive(&mut self, recursive: bool) {
        self.recursive = recursive;
    }

    pub fn set_dir(&mut self, dir: bool) {
        self.dir = dir;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn set_intractive(&mut self, intractive: Intractive) {
        self.intractive = intractive;
    }

    pub fn set_one_file_system(&mut self, one_file_system: bool) {
        self.one_file_system = one_file_system;
    }

    pub fn set_no_preserve_root(&mut self, no_preserve_root: bool) {
        self.no_preserve_root = no_preserve_root;
    }

    pub fn set_preserve_root(&mut self, preserve_root: bool) {
        self.preserve_root = preserve_root;
    }
}

#[derive(Debug, Clone)]
pub enum Intractive {
    Never,
    Once,
    Always,
}

impl Intractive {
    /// Returns `true` if the intractive is [`Never`].
    ///
    /// [`Never`]: Intractive::Never
    #[must_use]
    pub fn is_never(&self) -> bool {
        matches!(self, Self::Never)
    }

    /// Returns `true` if the intractive is [`Once`].
    ///
    /// [`Once`]: Intractive::Once
    #[must_use]
    pub fn is_once(&self) -> bool {
        matches!(self, Self::Once)
    }

    /// Returns `true` if the intractive is [`Always`].
    ///
    /// [`Always`]: Intractive::Always
    #[must_use]
    pub fn is_always(&self) -> bool {
        matches!(self, Self::Always)
    }
}

impl Default for Intractive {
    fn default() -> Self {
        Self::Always
    }
}
