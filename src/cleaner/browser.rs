//! Browser cache cleanup for Chrome, Edge and Firefox (per-user profiles).

use std::env;
use std::path::{Path, PathBuf};

use crate::cleaner::{Category, CategoryReport};
use crate::util::{clean_dir_contents, measure_dir};

/// A discovered cache directory and the browser it belongs to.
struct CacheDir {
    browser: &'static str,
    path: PathBuf,
}

fn local_app_data() -> Option<PathBuf> {
    env::var("LOCALAPPDATA").ok().map(PathBuf::from)
}

fn app_data() -> Option<PathBuf> {
    env::var("APPDATA").ok().map(PathBuf::from)
}

/// Collect cache directories across every Chromium/Firefox profile present.
fn cache_dirs() -> Vec<CacheDir> {
    let mut dirs = Vec::new();

    // Chromium-family browsers store one cache per profile under User Data.
    if let Some(local) = local_app_data() {
        let chromium = [
            (
                "Chrome",
                local.join("Google").join("Chrome").join("User Data"),
            ),
            (
                "Edge",
                local.join("Microsoft").join("Edge").join("User Data"),
            ),
            (
                "Brave",
                local
                    .join("BraveSoftware")
                    .join("Brave-Browser")
                    .join("User Data"),
            ),
        ];
        for (browser, user_data) in chromium {
            for profile in chromium_profiles(&user_data) {
                let cache = profile.join("Cache");
                if cache.exists() {
                    dirs.push(CacheDir {
                        browser,
                        path: cache,
                    });
                }
                let code_cache = profile.join("Code Cache");
                if code_cache.exists() {
                    dirs.push(CacheDir {
                        browser,
                        path: code_cache,
                    });
                }
            }
        }
    }

    // Firefox keeps caches under %LOCALAPPDATA%\Mozilla\Firefox\Profiles\*\cache2.
    if let Some(local) = local_app_data() {
        let profiles = local.join("Mozilla").join("Firefox").join("Profiles");
        for cache in firefox_caches(&profiles) {
            dirs.push(CacheDir {
                browser: "Firefox",
                path: cache,
            });
        }
    }
    // Older Firefox builds use %APPDATA%.
    if let Some(roaming) = app_data() {
        let profiles = roaming.join("Mozilla").join("Firefox").join("Profiles");
        for cache in firefox_caches(&profiles) {
            dirs.push(CacheDir {
                browser: "Firefox",
                path: cache,
            });
        }
    }

    dirs
}

/// Enumerate `Default` plus every `Profile N` directory under a Chromium
/// `User Data` folder.
fn chromium_profiles(user_data: &Path) -> Vec<PathBuf> {
    let mut profiles = Vec::new();
    if !user_data.exists() {
        return profiles;
    }
    let default = user_data.join("Default");
    if default.exists() {
        profiles.push(default);
    }
    if let Ok(entries) = std::fs::read_dir(user_data) {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with("Profile ") {
                profiles.push(entry.path());
            }
        }
    }
    profiles
}

/// Find every `cache2` directory beneath a Firefox `Profiles` folder.
fn firefox_caches(profiles: &Path) -> Vec<PathBuf> {
    let mut caches = Vec::new();
    if let Ok(entries) = std::fs::read_dir(profiles) {
        for entry in entries.filter_map(Result::ok) {
            let cache = entry.path().join("cache2");
            if cache.exists() {
                caches.push(cache);
            }
        }
    }
    caches
}

/// The cache directories for every detected browser profile, as plain paths.
pub fn cache_paths() -> Vec<PathBuf> {
    cache_dirs().into_iter().map(|c| c.path).collect()
}

pub fn scan() -> CategoryReport {
    let mut report = CategoryReport::new(Category::Browser);
    let dirs = cache_dirs();
    if dirs.is_empty() {
        report
            .notes
            .push("No supported browser caches found".to_string());
        return report;
    }
    for cache in &dirs {
        report.stats.merge(&measure_dir(&cache.path));
    }
    report.notes.push(found_note(&dirs));
    report
}

pub fn clean() -> CategoryReport {
    let mut report = CategoryReport::new(Category::Browser);
    let dirs = cache_dirs();
    for cache in &dirs {
        report.stats.merge(&clean_dir_contents(&cache.path));
    }
    if report.stats.skipped > 0 {
        report
            .notes
            .push("Some cache files were locked — close the browser and retry".to_string());
    }
    report
}

/// Summarise which browsers were detected, e.g. "Found: Chrome, Edge".
fn found_note(dirs: &[CacheDir]) -> String {
    let mut seen: Vec<&str> = Vec::new();
    for d in dirs {
        if !seen.contains(&d.browser) {
            seen.push(d.browser);
        }
    }
    format!("Found: {}", seen.join(", "))
}
