//! Large-file scanner: walk a tree and surface the biggest files.
//!
//! This category never deletes anything. Bulky files on `C:` are usually data
//! the user cares about (videos, VM images, installers), so we only *report*
//! them and let the user decide what to remove.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

/// One large file found during a scan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LargeFile {
    pub path: PathBuf,
    pub size: u64,
}

/// Walk `root` and return up to `limit` files of at least `min_size` bytes,
/// largest first.
pub fn scan(root: &Path, min_size: u64, limit: usize) -> Vec<LargeFile> {
    scan_roots(std::slice::from_ref(&root.to_path_buf()), min_size, limit)
}

/// Walk every directory in `roots` and return the largest files across all of
/// them, largest first.
///
/// Memory stays bounded: we keep a min-heap of at most `limit` entries and
/// discard anything smaller than the current smallest once full.
pub fn scan_roots(roots: &[PathBuf], min_size: u64, limit: usize) -> Vec<LargeFile> {
    let mut heap: BinaryHeap<Reverse<(u64, PathBuf)>> = BinaryHeap::new();

    for root in roots {
        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let size = match entry.metadata() {
                Ok(m) => m.len(),
                Err(_) => continue,
            };
            if size < min_size {
                continue;
            }

            if heap.len() < limit {
                heap.push(Reverse((size, entry.into_path())));
            } else if let Some(Reverse((smallest, _))) = heap.peek() {
                if size > *smallest {
                    heap.pop();
                    heap.push(Reverse((size, entry.into_path())));
                }
            }
        }
    }

    let mut files: Vec<LargeFile> = heap
        .into_iter()
        .map(|Reverse((size, path))| LargeFile { path, size })
        .collect();
    files.sort_by(|a, b| b.size.cmp(&a.size));
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_largest_above_threshold() {
        let dir = std::env::temp_dir().join("wclean_largefiles_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("small.bin"), vec![0u8; 10]).unwrap();
        fs::write(dir.join("big.bin"), vec![0u8; 5000]).unwrap();
        fs::write(dir.join("medium.bin"), vec![0u8; 2000]).unwrap();

        let found = scan(&dir, 1000, 10);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].size, 5000);
        assert_eq!(found[1].size, 2000);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn respects_limit() {
        let dir = std::env::temp_dir().join("wclean_largefiles_limit");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for i in 0..5 {
            fs::write(dir.join(format!("f{i}.bin")), vec![0u8; 1000 + i * 100]).unwrap();
        }
        let found = scan(&dir, 0, 2);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].size, 1400);
        assert_eq!(found[1].size, 1300);

        fs::remove_dir_all(&dir).unwrap();
    }
}
