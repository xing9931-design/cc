//! Recycle Bin support via the Win32 Shell API.
//!
//! On Windows we use `SHQueryRecycleBinW` to measure and `SHEmptyRecycleBinW`
//! to empty it. On other platforms these become no-ops so the crate still
//! builds and tests run, which keeps development possible off-Windows.

use crate::cleaner::{Category, CategoryReport};

#[cfg(windows)]
pub fn scan() -> CategoryReport {
    use windows::core::PCWSTR;
    use windows::Win32::UI::Shell::{SHQueryRecycleBinW, SHQUERYRBINFO};

    let mut report = CategoryReport::new(Category::RecycleBin);
    let mut info = SHQUERYRBINFO {
        cbSize: std::mem::size_of::<SHQUERYRBINFO>() as u32,
        i64Size: 0,
        i64NumItems: 0,
    };
    // A null root path queries the Recycle Bin across all drives.
    let result = unsafe { SHQueryRecycleBinW(PCWSTR::null(), &mut info) };
    match result {
        Ok(()) => {
            report.stats.bytes = info.i64Size.max(0) as u64;
            report.stats.files = info.i64NumItems.max(0) as u64;
        }
        Err(err) => report
            .notes
            .push(format!("Could not query Recycle Bin: {err}")),
    }
    report
}

#[cfg(windows)]
pub fn clean() -> CategoryReport {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::Shell::SHEmptyRecycleBinW;

    // No prompt, no progress dialog, no sound — we drive confirmation ourselves.
    const SHERB_NOCONFIRMATION: u32 = 0x0000_0001;
    const SHERB_NOPROGRESSUI: u32 = 0x0000_0002;
    const SHERB_NOSOUND: u32 = 0x0000_0004;

    // Measure first so we can report how much space was reclaimed.
    let measured = scan();
    let mut report = CategoryReport::new(Category::RecycleBin);

    let flags = SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND;
    let result = unsafe { SHEmptyRecycleBinW(HWND::default(), PCWSTR::null(), flags) };
    match result {
        Ok(()) => {
            report.stats.bytes = measured.stats.bytes;
            report.stats.files = measured.stats.files;
        }
        Err(err) => report
            .notes
            .push(format!("Could not empty Recycle Bin: {err}")),
    }
    report
}

#[cfg(not(windows))]
pub fn scan() -> CategoryReport {
    let mut report = CategoryReport::new(Category::RecycleBin);
    report
        .notes
        .push("Recycle Bin is only available on Windows".to_string());
    report
}

#[cfg(not(windows))]
pub fn clean() -> CategoryReport {
    scan()
}
