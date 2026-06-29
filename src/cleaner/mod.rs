//! Cleanup categories. Each category can be *scanned* (measure reclaimable
//! space without touching anything) and *cleaned* (actually free the space).

pub mod browser;
pub mod largefiles;
pub mod recyclebin;
pub mod temp;

use crate::util::DirStats;

/// Identifies a cleanup category on the command line and in reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Temp,
    Browser,
    RecycleBin,
}

impl Category {
    pub fn title(self) -> &'static str {
        match self {
            Category::Temp => "Temporary files",
            Category::Browser => "Browser caches",
            Category::RecycleBin => "Recycle Bin",
        }
    }

    pub fn all() -> &'static [Category] {
        &[Category::Temp, Category::Browser, Category::RecycleBin]
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
    }
}

/// Clean a category, reclaiming space. Returns what was actually freed.
pub fn clean(category: Category) -> CategoryReport {
    match category {
        Category::Temp => temp::clean(),
        Category::Browser => browser::clean(),
        Category::RecycleBin => recyclebin::clean(),
    }
}
