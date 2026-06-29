//! System-level junk: the Windows Update download cache and crash dumps.
//!
//! Both are directory-based, so they reuse the shared measure/clean helpers.
//! Targets resolve from environment variables, which keeps the logic testable
//! off-Windows even though the real paths only exist there.

use std::env;
use std::path::PathBuf;

use crate::cleaner::{Category, CategoryReport};
use crate::util::{clean_dir_contents, measure_dir};

/// Downloaded update payloads under `SoftwareDistribution\Download`.
pub fn windows_update_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(root) = env::var("SystemRoot").or_else(|_| env::var("windir")) {
        dirs.push(
            PathBuf::from(root)
                .join("SoftwareDistribution")
                .join("Download"),
        );
    }
    dirs
}

/// Crash dumps and Windows Error Reporting queues.
pub fn crash_dump_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(local) = env::var("LOCALAPPDATA") {
        let local = PathBuf::from(local);
        dirs.push(local.join("CrashDumps"));
        dirs.push(local.join("Microsoft").join("Windows").join("WER"));
    }
    if let Ok(root) = env::var("SystemRoot").or_else(|_| env::var("windir")) {
        dirs.push(PathBuf::from(root).join("Minidump"));
    }
    if let Ok(program_data) = env::var("ProgramData") {
        dirs.push(
            PathBuf::from(program_data)
                .join("Microsoft")
                .join("Windows")
                .join("WER"),
        );
    }
    dirs
}

fn scan_dirs(category: Category, dirs: &[PathBuf]) -> CategoryReport {
    let mut report = CategoryReport::new(category);
    let mut found = false;
    for dir in dirs {
        if dir.exists() {
            found = true;
            report.stats.merge(&measure_dir(dir));
        }
    }
    if !found {
        report
            .notes
            .push("Nothing found on this system".to_string());
    }
    report
}

fn clean_dirs(category: Category, dirs: &[PathBuf], admin_hint: bool) -> CategoryReport {
    let mut report = CategoryReport::new(category);
    for dir in dirs {
        if dir.exists() {
            report.stats.merge(&clean_dir_contents(dir));
        }
    }
    if report.stats.skipped > 0 {
        let hint = if admin_hint {
            " — try running as administrator"
        } else {
            ""
        };
        report.notes.push(format!(
            "{} item(s) in use and skipped{hint}",
            report.stats.skipped
        ));
    }
    report
}

pub fn windows_update_scan() -> CategoryReport {
    scan_dirs(Category::WindowsUpdate, &windows_update_dirs())
}

pub fn windows_update_clean() -> CategoryReport {
    clean_dirs(Category::WindowsUpdate, &windows_update_dirs(), true)
}

pub fn crash_dumps_scan() -> CategoryReport {
    scan_dirs(Category::CrashDumps, &crash_dump_dirs())
}

pub fn crash_dumps_clean() -> CategoryReport {
    clean_dirs(Category::CrashDumps, &crash_dump_dirs(), false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scan_reports_sizes_for_existing_dirs() {
        let base = std::env::temp_dir().join("wclean_system_test");
        let _ = fs::remove_dir_all(&base);
        let dir = base.join("Download");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("update.bin"), vec![0u8; 4096]).unwrap();

        let report = scan_dirs(Category::WindowsUpdate, std::slice::from_ref(&dir));
        assert_eq!(report.stats.bytes, 4096);
        assert_eq!(report.stats.files, 1);
        assert!(report.notes.is_empty());

        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn scan_notes_when_absent() {
        let missing = std::env::temp_dir().join("wclean_system_absent_xyz");
        let _ = fs::remove_dir_all(&missing);
        let report = scan_dirs(Category::CrashDumps, &[missing]);
        assert_eq!(report.stats.bytes, 0);
        assert_eq!(report.notes.len(), 1);
    }

    #[test]
    fn clean_empties_contents_but_keeps_dir() {
        let dir = std::env::temp_dir().join("wclean_system_clean_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a.dmp"), vec![0u8; 2048]).unwrap();

        let report = clean_dirs(Category::CrashDumps, std::slice::from_ref(&dir), false);
        assert_eq!(report.stats.bytes, 2048);
        assert!(dir.exists());
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 0);

        fs::remove_dir_all(&dir).unwrap();
    }
}
