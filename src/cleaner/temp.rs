//! Temporary-file cleanup: user TEMP, the system Windows\Temp, and Prefetch.

use std::env;
use std::path::PathBuf;

use crate::cleaner::{Category, CategoryReport};
use crate::util::{clean_dir_contents, measure_dir};

/// Resolve the set of temp directories worth cleaning on this machine.
///
/// On non-Windows hosts this resolves to whatever `TMPDIR`/`TEMP` point at, so
/// the logic stays testable; the real targets only exist on Windows.
fn temp_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut push = |p: PathBuf| {
        if !dirs.contains(&p) {
            dirs.push(p);
        }
    };

    // User temp (%TEMP% / %TMP% / TMPDIR).
    for var in ["TEMP", "TMP", "TMPDIR"] {
        if let Ok(val) = env::var(var) {
            if !val.is_empty() {
                push(PathBuf::from(val));
            }
        }
    }

    // System-wide temp and Prefetch live under the Windows directory.
    if let Ok(windir) = env::var("SystemRoot").or_else(|_| env::var("windir")) {
        let base = PathBuf::from(windir);
        push(base.join("Temp"));
        push(base.join("Prefetch"));
    }

    dirs
}

pub fn scan() -> CategoryReport {
    let mut report = CategoryReport::new(Category::Temp);
    let dirs = temp_dirs();
    if dirs.is_empty() {
        report
            .notes
            .push("No temp directories found (is TEMP set?)".to_string());
        return report;
    }
    for dir in dirs {
        if dir.exists() {
            report.stats.merge(&measure_dir(&dir));
        }
    }
    report
}

pub fn clean() -> CategoryReport {
    let mut report = CategoryReport::new(Category::Temp);
    for dir in temp_dirs() {
        if dir.exists() {
            report.stats.merge(&clean_dir_contents(&dir));
        }
    }
    if report.stats.skipped > 0 {
        report.notes.push(format!(
            "{} item(s) in use and skipped — close apps or run as admin",
            report.stats.skipped
        ));
    }
    report
}
