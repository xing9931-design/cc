//! Command-line interface definition (clap derive).

use clap::{Parser, Subcommand};

use wclean::cleaner::Category;

#[derive(Parser, Debug)]
#[command(
    name = "wclean",
    version,
    about = "A safe, fast C: drive cleanup tool for Windows",
    long_about = "wclean scans and reclaims space from temporary files, browser \
                  caches and the Recycle Bin, and can report the largest files \
                  on a drive. Cleaning is opt-in and asks for confirmation \
                  unless you pass --yes."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Measure reclaimable space without deleting anything.
    Scan {
        /// Limit to specific categories (default: all).
        #[arg(value_enum)]
        categories: Vec<CategoryArg>,
    },

    /// Reclaim space. Asks for confirmation unless --yes is given.
    Clean {
        /// Limit to specific categories (default: all).
        #[arg(value_enum)]
        categories: Vec<CategoryArg>,

        /// Show what would be deleted, then stop without deleting.
        #[arg(long)]
        dry_run: bool,

        /// Skip the confirmation prompt.
        #[arg(short, long)]
        yes: bool,
    },

    /// List the largest files under a path (report only, never deletes).
    Large {
        /// Directory to scan (default: the system drive, e.g. C:\).
        #[arg(default_value_t = default_root())]
        path: String,

        /// Only show files at least this many megabytes.
        #[arg(long, default_value_t = 100)]
        min_mb: u64,

        /// Maximum number of files to list.
        #[arg(long, default_value_t = 20)]
        top: usize,
    },
}

/// Categories selectable on the command line.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum CategoryArg {
    Temp,
    Browser,
    Recyclebin,
    WindowsUpdate,
    CrashDumps,
}

impl From<CategoryArg> for Category {
    fn from(arg: CategoryArg) -> Self {
        match arg {
            CategoryArg::Temp => Category::Temp,
            CategoryArg::Browser => Category::Browser,
            CategoryArg::Recyclebin => Category::RecycleBin,
            CategoryArg::WindowsUpdate => Category::WindowsUpdate,
            CategoryArg::CrashDumps => Category::CrashDumps,
        }
    }
}

/// Resolve which categories to act on, defaulting to all when none are given.
pub fn resolve_categories(args: &[CategoryArg]) -> Vec<Category> {
    if args.is_empty() {
        Category::all().to_vec()
    } else {
        args.iter().map(|a| Category::from(*a)).collect()
    }
}

/// Best guess at the system drive root for the default `large` scan path.
fn default_root() -> String {
    if cfg!(windows) {
        std::env::var("SystemDrive")
            .map(|d| format!("{d}\\"))
            .unwrap_or_else(|_| "C:\\".to_string())
    } else {
        "/".to_string()
    }
}
