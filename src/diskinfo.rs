//! Disk capacity for the system drive — the data behind the usage gauge.
//!
//! On Windows this calls `GetDiskFreeSpaceExW`. On other platforms it returns
//! `None` so the UI can degrade gracefully during off-Windows development.

/// Total and free bytes for a drive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiskUsage {
    pub total: u64,
    pub free: u64,
}

impl DiskUsage {
    pub fn used(&self) -> u64 {
        self.total.saturating_sub(self.free)
    }

    /// Fraction of the drive in use, in `0.0..=1.0`.
    pub fn used_fraction(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.used() as f64 / self.total as f64) as f32
        }
    }

    pub fn percent_used(&self) -> u32 {
        (self.used_fraction() * 100.0).round() as u32
    }
}

/// Query usage for the drive Windows is installed on (typically `C:`).
#[cfg(windows)]
pub fn system_drive() -> Option<DiskUsage> {
    let root = std::env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
    usage_for(&format!("{root}\\"))
}

/// Query usage for a specific drive root, e.g. `"C:\\"`.
#[cfg(windows)]
pub fn usage_for(root: &str) -> Option<DiskUsage> {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

    // Build a NUL-terminated wide string and keep it alive for the call.
    let wide: Vec<u16> = std::ffi::OsStr::new(root)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut free_to_caller: u64 = 0;
    let mut total: u64 = 0;
    let mut total_free: u64 = 0;
    let ok = unsafe {
        GetDiskFreeSpaceExW(
            PCWSTR(wide.as_ptr()),
            Some(&mut free_to_caller),
            Some(&mut total),
            Some(&mut total_free),
        )
    };
    match ok {
        Ok(()) if total > 0 => Some(DiskUsage {
            total,
            // Free-to-caller respects user quotas; it's the honest "free" figure.
            free: free_to_caller,
        }),
        _ => None,
    }
}

#[cfg(not(windows))]
pub fn system_drive() -> Option<DiskUsage> {
    // Off-Windows there is no C: drive. To aid development and screenshots, a
    // demo value can be injected via `WCLEAN_DEMO_DISK=freeGB:totalGB`.
    let spec = std::env::var("WCLEAN_DEMO_DISK").ok()?;
    let (free_gb, total_gb) = spec.split_once(':')?;
    let gb = 1024u64 * 1024 * 1024;
    Some(DiskUsage {
        free: free_gb.trim().parse::<u64>().ok()? * gb,
        total: total_gb.trim().parse::<u64>().ok()? * gb,
    })
}

#[cfg(not(windows))]
pub fn usage_for(_root: &str) -> Option<DiskUsage> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fractions_and_percent() {
        let u = DiskUsage {
            total: 1000,
            free: 250,
        };
        assert_eq!(u.used(), 750);
        assert_eq!(u.percent_used(), 75);
        assert!((u.used_fraction() - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn empty_disk_is_safe() {
        let u = DiskUsage { total: 0, free: 0 };
        assert_eq!(u.used_fraction(), 0.0);
        assert_eq!(u.percent_used(), 0);
    }
}
