//! Cleanup categories. Each category can be *scanned* (measure reclaimable
//! space without touching anything) and *cleaned* (actually free the space).

pub mod browser;
pub mod largefiles;
pub mod recyclebin;
pub mod system;
pub mod temp;

use crate::util::DirStats;

/// How safe a category is to clean, surfaced to the user as a trust signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    /// Regenerated automatically; cleaning has no downside.
    Safe,
    /// Cleaning may sign you out of sites or slow the next launch briefly.
    Mild,
}

impl Risk {
    pub fn label(self) -> &'static str {
        match self {
            Risk::Safe => "Safe",
            Risk::Mild => "Low impact",
        }
    }
}

/// Identifies a cleanup category on the command line and in reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Temp,
    Browser,
    RecycleBin,
    WindowsUpdate,
    CrashDumps,
}

impl Category {
    pub fn title(self) -> &'static str {
        match self {
            Category::Temp => "Temporary files",
            Category::Browser => "Browser caches",
            Category::RecycleBin => "Recycle Bin",
            Category::WindowsUpdate => "Windows Update cache",
            Category::CrashDumps => "Crash dumps & error reports",
        }
    }

    /// One-line, plain-language explanation of what cleaning this removes.
    pub fn description(self) -> &'static str {
        match self {
            Category::Temp => "Leftover files from Windows and apps. Safe to remove.",
            Category::Browser => "Cached images and files from your browsers.",
            Category::RecycleBin => "Permanently delete everything in the Recycle Bin.",
            Category::WindowsUpdate => "Old downloaded update files. May need admin rights.",
            Category::CrashDumps => "Memory dumps and error reports. Safe to remove.",
        }
    }

    /// A glyph used as the category's icon in the UI.
    pub fn glyph(self) -> &'static str {
        match self {
            Category::Temp => "📄",
            Category::Browser => "🌐",
            Category::RecycleBin => "🗑",
            Category::WindowsUpdate => "📦",
            Category::CrashDumps => "📁",
        }
    }

    pub fn risk(self) -> Risk {
        match self {
            Category::Temp => Risk::Safe,
            Category::Browser => Risk::Mild,
            Category::RecycleBin => Risk::Mild,
            Category::WindowsUpdate => Risk::Mild,
            Category::CrashDumps => Risk::Safe,
        }
    }

    /// A stable identifier used for config persistence and the CLI.
    pub fn key(self) -> &'static str {
        match self {
            Category::Temp => "temp",
            Category::Browser => "browser",
            Category::RecycleBin => "recyclebin",
            Category::WindowsUpdate => "windows-update",
            Category::CrashDumps => "crash-dumps",
        }
    }

    /// Parse a [`Category::key`] back into a category.
    pub fn from_key(key: &str) -> Option<Category> {
        Category::all().iter().copied().find(|c| c.key() == key)
    }

    pub fn all() -> &'static [Category] {
        &[
            Category::Temp,
            Category::Browser,
            Category::RecycleBin,
            Category::WindowsUpdate,
            Category::CrashDumps,
        ]
    }
}

/// Result of scanning or cleaning a single category.
#[derive(Debug, Clone)]
pub struct CategoryReport {
    pub category: Category,
    pub stats: DirStats,
    /// Human-readable notes (e.g. "Chrome not installed", "run as admin").
    pub notes: Vec<String>,
}

impl CategoryReport {
    pub fn new(category: Category) -> Self {
        Self {
            category,
            stats: DirStats::default(),
            notes: Vec::new(),
        }
    }
}

/// Scan a category, reporting reclaimable space without deleting anything.
pub fn scan(category: Category) -> CategoryReport {
    match category {
        Category::Temp => temp::scan(),
        Category::Browser => browser::scan(),
        Category::RecycleBin => recyclebin::scan(),
        Category::WindowsUpdate => system::windows_update_scan(),
        Category::CrashDumps => system::crash_dumps_scan(),
    }
}

/// Clean a category, reclaiming space. Returns what was actually freed.
pub fn clean(category: Category) -> CategoryReport {
    match category {
        Category::Temp => temp::clean(),
        Category::Browser => browser::clean(),
        Category::RecycleBin => recyclebin::clean(),
        Category::WindowsUpdate => system::windows_update_clean(),
        Category::CrashDumps => system::crash_dumps_clean(),
    }
}
