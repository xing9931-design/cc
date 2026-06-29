//! Shared helpers: byte formatting, directory sizing, and safe deletion.

use std::fs;
use std::io;
use std::path::Path;

use walkdir::WalkDir;

/// Format a byte count into a human-readable string (e.g. `1.5 GB`).
pub fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{size:.2} {}", UNITS[unit])
    }
}

/// Outcome of walking and (optionally) deleting the contents of a directory.
#[derive(Default, Debug, Clone)]
pub struct DirStats {
    /// Total bytes covered by regular files.
    pub bytes: u64,
    /// Number of regular files seen.
    pub files: u64,
    /// Number of entries that could not be deleted (in use, locked, denied).
    pub skipped: u64,
}

impl DirStats {
    pub fn merge(&mut self, other: &DirStats) {
        self.bytes += other.bytes;
        self.files += other.files;
        self.skipped += other.skipped;
    }
}

/// Sum the size of every regular file beneath `root` without deleting anything.
pub fn measure_dir(root: &Path) -> DirStats {
    let mut stats = DirStats::default();
    if !root.exists() {
        return stats;
    }
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            if let Ok(meta) = entry.metadata() {
                stats.bytes += meta.len();
                stats.files += 1;
            }
        }
    }
    stats
}

/// Delete the *contents* of `root`, keeping `root` itself in place.
///
/// Files that are locked or in use are skipped and counted; they never abort
/// the whole operation. Returns the bytes/files actually reclaimed.
pub fn clean_dir_contents(root: &Path) -> DirStats {
    let mut stats = DirStats::default();
    let read_dir = match fs::read_dir(root) {
        Ok(rd) => rd,
        Err(_) => return stats,
    };

    for entry in read_dir.filter_map(Result::ok) {
        let path = entry.path();
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => {
                stats.skipped += 1;
                continue;
            }
        };

        if meta.is_dir() {
            let sub = measure_dir(&path);
            match fs::remove_dir_all(&path) {
                Ok(()) => stats.merge(&sub),
                Err(_) => {
                    // Partial failure (some files in use): re-measure what's left.
                    let remaining = measure_dir(&path);
                    stats.bytes += sub.bytes.saturating_sub(remaining.bytes);
                    stats.files += sub.files.saturating_sub(remaining.files);
                    stats.skipped += 1;
                }
            }
        } else {
            let len = meta.len();
            match remove_file(&path) {
                Ok(()) => {
                    stats.bytes += len;
                    stats.files += 1;
                }
                Err(_) => stats.skipped += 1,
            }
        }
    }
    stats
}

/// Remove a file, clearing a read-only attribute first if necessary.
fn remove_file(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(_) => {
            // Try clearing the read-only flag, then retry once.
            if let Ok(meta) = fs::metadata(path) {
                let mut perms = meta.permissions();
                #[allow(clippy::permissions_set_readonly_false)]
                perms.set_readonly(false);
                let _ = fs::set_permissions(path, perms);
            }
            fs::remove_file(path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_bytes_scales() {
        assert_eq!(human_bytes(0), "0 B");
        assert_eq!(human_bytes(512), "512 B");
        assert_eq!(human_bytes(1024), "1.00 KB");
        assert_eq!(human_bytes(1536), "1.50 KB");
        assert_eq!(human_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(human_bytes(3 * 1024 * 1024 * 1024), "3.00 GB");
    }
}
