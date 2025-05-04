#![deny(
    rust_2018_idioms,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::redundant_clone
)]

mod remove;
pub use remove::*;
mod macros;
mod config;
pub use config::*;
mod log;
pub use log::*;
mod errors;
pub use errors::*;
