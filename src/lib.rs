//! wclean core library: disk-cleanup logic shared by the CLI and GUI binaries.
//!
//! The [`cleaner`] module exposes scan/clean operations per category; [`util`]
//! provides byte formatting and safe deletion helpers.

pub mod cleaner;
pub mod config;
pub mod diskinfo;
pub mod usage;
pub mod util;

#[cfg(feature = "gui")]
pub mod gui;
