//! wclean-gui — a graphical front-end for the wclean disk cleaner.
//!
//! Built on egui/eframe. Long-running scans and cleans run on a background
//! thread and report back over a channel, so the window never freezes.

#![cfg_attr(windows, windows_subsystem = "windows")]

use std::sync::mpsc::{channel, Receiver};
use std::thread;

use eframe::egui;

use wclean::cleaner::largefiles::{self, LargeFile};
use wclean::cleaner::{self, Category, CategoryReport};
use wclean::util::human_bytes;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([580.0, 680.0])
            .with_min_inner_size([460.0, 480.0])
            .with_title("wclean — C: Drive Cleanup"),
        ..Default::default()
    };
    eframe::run_native(
        "wclean",
        options,
        Box::new(|_cc| Ok(Box::new(WCleanApp::default()))),
    )
}

/// A finished background job, delivered to the UI thread.
enum Job {
    Scan(Vec<CategoryReport>),
    Clean(Vec<CategoryReport>),
    Large(Vec<LargeFile>),
}

struct WCleanApp {
    sel_temp: bool,
    sel_browser: bool,
    sel_recycle: bool,

    scan: Vec<CategoryReport>,
    cleaned: Vec<CategoryReport>,

    large_path: String,
    large_min_mb: u64,
    large_top: usize,
    large: Vec<LargeFile>,

    status: String,
    busy: bool,
    rx: Option<Receiver<Job>>,
    confirm_clean: bool,
}

impl Default for WCleanApp {
    fn default() -> Self {
        Self {
            sel_temp: true,
            sel_browser: true,
            sel_recycle: true,
            scan: Vec::new(),
            cleaned: Vec::new(),
            large_path: default_root(),
            large_min_mb: 100,
            large_top: 20,
            large: Vec::new(),
            status: "Ready. Choose categories and Scan.".to_string(),
            busy: false,
            rx: None,
            confirm_clean: false,
        }
    }
}

impl WCleanApp {
    fn selected(&self) -> Vec<Category> {
        let mut cats = Vec::new();
        if self.sel_temp {
            cats.push(Category::Temp);
        }
        if self.sel_browser {
            cats.push(Category::Browser);
        }
        if self.sel_recycle {
            cats.push(Category::RecycleBin);
        }
        cats
    }

    /// Run `f` on a worker thread; its result arrives via `self.rx`.
    fn spawn<F>(&mut self, ctx: &egui::Context, status: &str, f: F)
    where
        F: FnOnce() -> Job + Send + 'static,
    {
        let (tx, rx) = channel();
        self.rx = Some(rx);
        self.busy = true;
        self.status = status.to_string();
        let ctx = ctx.clone();
        thread::spawn(move || {
            let job = f();
            let _ = tx.send(job);
            ctx.request_repaint();
        });
    }

    /// Drain a finished job, if any, and update state.
    fn poll(&mut self) {
        let done = self.rx.as_ref().and_then(|rx| rx.try_recv().ok());
        if let Some(job) = done {
            self.busy = false;
            self.rx = None;
            match job {
                Job::Scan(reports) => {
                    let total: u64 = reports.iter().map(|r| r.stats.bytes).sum();
                    self.status = format!("Scan complete — {} reclaimable.", human_bytes(total));
                    self.scan = reports;
                    self.cleaned.clear();
                }
                Job::Clean(reports) => {
                    let total: u64 = reports.iter().map(|r| r.stats.bytes).sum();
                    self.status = format!("Done — freed {}.", human_bytes(total));
                    self.cleaned = reports;
                    self.scan.clear();
                }
                Job::Large(files) => {
                    self.status = format!("Found {} large file(s).", files.len());
                    self.large = files;
                }
            }
        }
    }
}

impl eframe::App for WCleanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll();

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.heading("🧹 wclean");
                ui.label(egui::RichText::new("C: Drive Cleanup").weak());
            });
            ui.add_space(6.0);
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if self.busy {
                    ui.spinner();
                }
                ui.label(&self.status);
            });
            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.cleanup_section(ui, ctx);
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);
                self.large_section(ui, ctx);
            });
        });

        self.confirm_dialog(ctx);
    }
}

impl WCleanApp {
    fn cleanup_section(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Cleanup");
        ui.add_space(4.0);
        ui.label("Select what to clean:");
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.sel_temp, "Temporary files");
            ui.checkbox(&mut self.sel_browser, "Browser caches");
            ui.checkbox(&mut self.sel_recycle, "Recycle Bin");
        });

        ui.add_space(8.0);
        let any_selected = !self.selected().is_empty();
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.busy && any_selected, egui::Button::new("🔍  Scan"))
                .clicked()
            {
                let cats = self.selected();
                self.spawn(ctx, "Scanning…", move || {
                    Job::Scan(cats.iter().map(|&c| cleaner::scan(c)).collect())
                });
            }

            let can_clean = !self.busy && any_selected;
            if ui
                .add_enabled(can_clean, egui::Button::new("🗑  Clean selected…"))
                .clicked()
            {
                self.confirm_clean = true;
            }
        });

        // Results table (scan preview or clean outcome).
        let (reports, freed) = if !self.cleaned.is_empty() {
            (&self.cleaned, true)
        } else {
            (&self.scan, false)
        };
        if !reports.is_empty() {
            ui.add_space(10.0);
            egui::Grid::new("results")
                .num_columns(3)
                .striped(true)
                .spacing([16.0, 6.0])
                .show(ui, |ui| {
                    ui.strong("Category");
                    ui.strong(if freed { "Freed" } else { "Reclaimable" });
                    ui.strong("Files");
                    ui.end_row();
                    let mut total = 0u64;
                    for r in reports {
                        ui.label(r.category.title());
                        ui.label(
                            egui::RichText::new(human_bytes(r.stats.bytes))
                                .color(egui::Color32::from_rgb(0x3c, 0xb3, 0x71)),
                        );
                        ui.label(r.stats.files.to_string());
                        ui.end_row();
                        total += r.stats.bytes;
                    }
                    ui.strong("Total");
                    ui.strong(human_bytes(total));
                    ui.label("");
                    ui.end_row();
                });

            // Surface any notes (browsers found, skipped files, etc.).
            for r in reports {
                for note in &r.notes {
                    ui.label(egui::RichText::new(format!("• {note}")).weak().small());
                }
            }
        }
    }

    fn large_section(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Large files");
        ui.add_space(4.0);
        ui.label("Find the biggest files under a folder (report only — nothing is deleted).");
        ui.add_space(6.0);

        egui::Grid::new("large_opts")
            .num_columns(2)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                ui.label("Folder:");
                ui.add(egui::TextEdit::singleline(&mut self.large_path).desired_width(360.0));
                ui.end_row();
                ui.label("Min size (MB):");
                ui.add(egui::DragValue::new(&mut self.large_min_mb).range(1..=1_000_000));
                ui.end_row();
                ui.label("Show top:");
                ui.add(egui::DragValue::new(&mut self.large_top).range(1..=1000));
                ui.end_row();
            });

        ui.add_space(6.0);
        if ui
            .add_enabled(!self.busy, egui::Button::new("📁  Find large files"))
            .clicked()
        {
            let path = std::path::PathBuf::from(self.large_path.clone());
            let min = self.large_min_mb.saturating_mul(1024 * 1024);
            let top = self.large_top;
            self.spawn(ctx, "Scanning for large files…", move || {
                Job::Large(largefiles::scan(&path, min, top))
            });
        }

        if !self.large.is_empty() {
            ui.add_space(10.0);
            egui::Grid::new("large_results")
                .num_columns(2)
                .striped(true)
                .spacing([16.0, 4.0])
                .show(ui, |ui| {
                    ui.strong("Size");
                    ui.strong("Path");
                    ui.end_row();
                    for f in &self.large {
                        ui.label(
                            egui::RichText::new(human_bytes(f.size))
                                .color(egui::Color32::from_rgb(0x3c, 0xb3, 0x71)),
                        );
                        ui.label(f.path.display().to_string());
                        ui.end_row();
                    }
                });
        }
    }

    fn confirm_dialog(&mut self, ctx: &egui::Context) {
        if !self.confirm_clean {
            return;
        }
        let mut open = true;
        egui::Window::new("Confirm cleanup")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .open(&mut open)
            .show(ctx, |ui| {
                let cats = self.selected();
                ui.label("The following will be permanently deleted:");
                ui.add_space(4.0);
                for c in &cats {
                    ui.label(format!("  • {}", c.title()));
                }
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.confirm_clean = false;
                    }
                    let delete = ui.add(
                        egui::Button::new(
                            egui::RichText::new("Delete").color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(0xc0, 0x39, 0x2b)),
                    );
                    if delete.clicked() {
                        self.confirm_clean = false;
                        let cats = cats.clone();
                        self.spawn(ctx, "Cleaning…", move || {
                            Job::Clean(cats.iter().map(|&c| cleaner::clean(c)).collect())
                        });
                    }
                });
            });
        if !open {
            self.confirm_clean = false;
        }
    }
}

/// Best guess at the system drive root for the default large-file scan path.
fn default_root() -> String {
    if cfg!(windows) {
        std::env::var("SystemDrive")
            .map(|d| format!("{d}\\"))
            .unwrap_or_else(|_| "C:\\".to_string())
    } else {
        "/".to_string()
    }
}
