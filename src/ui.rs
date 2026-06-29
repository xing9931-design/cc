//! Terminal output helpers: report tables, prompts and colored summaries.

use std::io::{self, Write};

use owo_colors::OwoColorize;

use crate::cleaner::largefiles::LargeFile;
use crate::cleaner::CategoryReport;
use crate::util::human_bytes;

pub fn banner(action: &str) {
    println!("{} {}", "wclean".bold().cyan(), action.dimmed());
    println!();
}

/// Print one category line, e.g. `Temporary files   1.20 GB  (3,201 files)`.
pub fn category_line(report: &CategoryReport) {
    let title = report.category.title();
    let size = human_bytes(report.stats.bytes);
    print!(
        "  {:<20} {:>12}  {}",
        title,
        size.green().to_string(),
        format!("({} files)", thousands(report.stats.files)).dimmed()
    );
    println!();
    for note in &report.notes {
        println!("      {} {}", "·".dimmed(), note.dimmed());
    }
}

/// Print the grand total across all categories.
pub fn total_line(bytes: u64, label: &str) {
    println!();
    println!(
        "  {:<20} {:>12}",
        label.bold(),
        human_bytes(bytes).bold().green().to_string()
    );
}

/// Render the large-file report as a ranked list.
pub fn large_files(files: &[LargeFile], root: &str) {
    if files.is_empty() {
        println!("  {}", "No files above the size threshold.".dimmed());
        return;
    }
    println!("  {}", format!("Largest files under {root}:").dimmed());
    println!();
    for (i, f) in files.iter().enumerate() {
        println!(
            "  {:>2}. {:>10}  {}",
            (i + 1).dimmed(),
            human_bytes(f.size).green().to_string(),
            f.path.display()
        );
    }
}

/// Ask a yes/no question on stdin. Returns `false` on EOF or anything but "y".
pub fn confirm(question: &str) -> bool {
    print!("{} {} ", question, "[y/N]".dimmed());
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

pub fn warn(msg: &str) {
    eprintln!("{} {}", "warning:".yellow().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "›".cyan(), msg);
}

/// Format an integer with thousands separators (1234567 -> "1,234,567").
fn thousands(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    let len = bytes.len();
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*b as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::thousands;

    #[test]
    fn thousands_groups() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(1000), "1,000");
        assert_eq!(thousands(1234567), "1,234,567");
    }
}
