//! wclean — a safe, fast C: drive cleanup tool for Windows.

mod cli;
mod ui;

use clap::Parser;

use cli::{Cli, Command};
use wclean::cleaner;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Scan { categories } => run_scan(&categories),
        Command::Clean {
            categories,
            dry_run,
            yes,
        } => run_clean(&categories, dry_run, yes),
        Command::Large { path, min_mb, top } => run_large(&path, min_mb, top),
    }
}

fn run_scan(categories: &[cli::CategoryArg]) {
    ui::banner("scan — measuring reclaimable space");
    let cats = cli::resolve_categories(categories);
    let mut total = 0u64;
    for cat in cats {
        let report = cleaner::scan(cat);
        total += report.stats.bytes;
        ui::category_line(&report);
    }
    ui::total_line(total, "Reclaimable");
    println!();
    ui::info("Run `wclean clean` to free this space.");
}

fn run_clean(categories: &[cli::CategoryArg], dry_run: bool, yes: bool) {
    let cats = cli::resolve_categories(categories);

    // Always scan first so the user sees what is at stake.
    ui::banner(if dry_run {
        "clean --dry-run — nothing will be deleted"
    } else {
        "clean — about to reclaim space"
    });

    let mut total = 0u64;
    for &cat in &cats {
        let report = cleaner::scan(cat);
        total += report.stats.bytes;
        ui::category_line(&report);
    }
    ui::total_line(total, "Reclaimable");
    println!();

    if dry_run {
        ui::info("Dry run complete — no files were deleted.");
        return;
    }

    if total == 0 {
        ui::info("Nothing to clean. You're all tidy.");
        return;
    }

    if !yes && !ui::confirm("Delete the items above?") {
        ui::info("Cancelled — nothing was deleted.");
        return;
    }

    println!();
    let mut freed = 0u64;
    for cat in cats {
        let report = cleaner::clean(cat);
        freed += report.stats.bytes;
        ui::category_line(&report);
    }
    ui::total_line(freed, "Freed");
    println!();
    ui::info("Done.");
}

fn run_large(path: &str, min_mb: u64, top: usize) {
    ui::banner("large — scanning for big files");
    let root = std::path::Path::new(path);
    if !root.exists() {
        ui::warn(&format!("path does not exist: {path}"));
        return;
    }
    let min_size = min_mb.saturating_mul(1024 * 1024);
    ui::info(&format!(
        "Scanning {path} for files ≥ {min_mb} MB (this can take a while)…"
    ));
    println!();
    let files = cleaner::largefiles::scan(root, min_size, top);
    ui::large_files(&files, path);
    println!();
    ui::info("These are reported only — wclean never deletes your files here.");
}
