//! Storage breakdown: "where is my space going?"
//!
//! Given a folder, measure the total size of each immediate sub-folder (plus
//! the loose files directly inside it) so the UI can show what dominates.

use std::path::Path;

use crate::util::measure_dir;

/// One entry in a storage breakdown: a sub-folder, or the loose-files bucket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub size: u64,
    /// True for the synthetic "files directly in this folder" bucket.
    pub is_loose_files: bool,
}

/// Measure `root`'s immediate children and return the largest entries, biggest
/// first. Loose files directly under `root` are aggregated into one bucket.
///
/// This walks the whole tree once, so it can be slow on a full drive — callers
/// should run it off the UI thread.
pub fn breakdown(root: &Path, limit: usize) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut loose: u64 = 0;

    let read = match std::fs::read_dir(root) {
        Ok(rd) => rd,
        Err(_) => return entries,
    };

    for item in read.filter_map(Result::ok) {
        let meta = match item.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.is_dir() {
            let size = measure_dir(&item.path()).bytes;
            if size > 0 {
                entries.push(Entry {
                    name: item.file_name().to_string_lossy().into_owned(),
                    size,
                    is_loose_files: false,
                });
            }
        } else if meta.is_file() {
            loose += meta.len();
        }
    }

    if loose > 0 {
        entries.push(Entry {
            name: "(files here)".to_string(),
            size: loose,
            is_loose_files: true,
        });
    }

    entries.sort_by(|a, b| b.size.cmp(&a.size));
    entries.truncate(limit);
    entries
}

/// Sum of all entry sizes — the denominator for proportion bars.
pub fn total(entries: &[Entry]) -> u64 {
    entries.iter().map(|e| e.size).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn ranks_subfolders_and_buckets_loose_files() {
        let root = std::env::temp_dir().join("wclean_usage_test");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("big")).unwrap();
        fs::create_dir_all(root.join("small")).unwrap();
        fs::write(root.join("big").join("a.bin"), vec![0u8; 5000]).unwrap();
        fs::write(root.join("small").join("b.bin"), vec![0u8; 1000]).unwrap();
        fs::write(root.join("loose.bin"), vec![0u8; 2000]).unwrap();

        let entries = breakdown(&root, 10);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].name, "big");
        assert_eq!(entries[0].size, 5000);
        // Loose files (2000) rank above "small" (1000).
        assert!(entries[1].is_loose_files);
        assert_eq!(entries[1].size, 2000);
        assert_eq!(entries[2].name, "small");
        assert_eq!(total(&entries), 8000);

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn empty_for_missing_root() {
        let missing = std::env::temp_dir().join("wclean_usage_missing_zzz");
        let _ = fs::remove_dir_all(&missing);
        assert!(breakdown(&missing, 10).is_empty());
    }
}
