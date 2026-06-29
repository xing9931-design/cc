//! Lightweight, dependency-free preference persistence.
//!
//! Settings are stored as `key=value` lines under the user's config directory
//! (`%APPDATA%\wclean\config` on Windows, `~/.config/wclean/config` elsewhere).
//! Unknown keys are ignored, so the format can evolve without breaking.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

/// Persisted user preferences. All fields are optional; missing keys fall back
/// to in-app defaults.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Config {
    /// `"dark"` or `"light"`.
    pub theme: Option<String>,
    /// Selected category keys (see `Category::key`).
    pub selected: Option<Vec<String>>,
    pub large_path: Option<String>,
    pub large_min_mb: Option<u64>,
    pub large_top: Option<usize>,
    /// Lifetime bytes reclaimed across all cleans.
    pub total_freed: Option<u64>,
    /// Lifetime number of cleanups performed.
    pub clean_count: Option<u64>,
}

impl Config {
    /// Parse from the `key=value` text format. Tolerant of blank/garbage lines.
    pub fn parse(text: &str) -> Config {
        let mut map: BTreeMap<&str, &str> = BTreeMap::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                map.insert(k.trim(), v.trim());
            }
        }
        Config {
            theme: map.get("theme").map(|s| s.to_string()),
            selected: map.get("selected").map(|s| {
                s.split(',')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect()
            }),
            large_path: map.get("large_path").map(|s| s.to_string()),
            large_min_mb: map.get("large_min_mb").and_then(|s| s.parse().ok()),
            large_top: map.get("large_top").and_then(|s| s.parse().ok()),
            total_freed: map.get("total_freed").and_then(|s| s.parse().ok()),
            clean_count: map.get("clean_count").and_then(|s| s.parse().ok()),
        }
    }

    /// Serialize to the `key=value` text format.
    pub fn to_text(&self) -> String {
        let mut out = String::from("# wclean preferences\n");
        if let Some(t) = &self.theme {
            out.push_str(&format!("theme={t}\n"));
        }
        if let Some(sel) = &self.selected {
            out.push_str(&format!("selected={}\n", sel.join(",")));
        }
        if let Some(p) = &self.large_path {
            out.push_str(&format!("large_path={p}\n"));
        }
        if let Some(m) = self.large_min_mb {
            out.push_str(&format!("large_min_mb={m}\n"));
        }
        if let Some(t) = self.large_top {
            out.push_str(&format!("large_top={t}\n"));
        }
        if let Some(f) = self.total_freed {
            out.push_str(&format!("total_freed={f}\n"));
        }
        if let Some(c) = self.clean_count {
            out.push_str(&format!("clean_count={c}\n"));
        }
        out
    }

    /// Load from disk, returning defaults if absent or unreadable.
    pub fn load() -> Config {
        config_path()
            .and_then(|p| fs::read_to_string(p).ok())
            .map(|text| Config::parse(&text))
            .unwrap_or_default()
    }

    /// Best-effort save. Failures are swallowed — preferences are non-critical.
    pub fn save(&self) {
        if let Some(path) = config_path() {
            if let Some(dir) = path.parent() {
                let _ = fs::create_dir_all(dir);
            }
            let _ = fs::write(path, self.to_text());
        }
    }
}

/// The path to the wclean config file, if a config directory can be resolved.
fn config_path() -> Option<PathBuf> {
    let base = if cfg!(windows) {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
    };
    base.map(|b| b.join("wclean").join("config"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let cfg = Config {
            theme: Some("light".into()),
            selected: Some(vec!["temp".into(), "browser".into()]),
            large_path: Some("C:\\".into()),
            large_min_mb: Some(250),
            large_top: Some(30),
            total_freed: Some(9_876_543_210),
            clean_count: Some(42),
        };
        let parsed = Config::parse(&cfg.to_text());
        assert_eq!(parsed, cfg);
    }

    #[test]
    fn tolerates_garbage_and_blanks() {
        let text = "# comment\n\n  theme = dark \nnonsense line\nlarge_top=15\n";
        let cfg = Config::parse(text);
        assert_eq!(cfg.theme.as_deref(), Some("dark"));
        assert_eq!(cfg.large_top, Some(15));
        assert!(cfg.selected.is_none());
    }

    #[test]
    fn empty_is_default() {
        assert_eq!(Config::parse(""), Config::default());
    }
}
