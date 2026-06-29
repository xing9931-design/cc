//! Application state and screen layout for the wclean GUI.

use std::sync::mpsc::{channel, Receiver};
use std::thread;

use eframe::egui::{self, Align, Layout, RichText};

use super::theme::{self, space, Mode};
use super::widgets;
use crate::cleaner::largefiles::{self, LargeFile};
use crate::cleaner::{self, Category, CategoryReport};
use crate::config::Config;
use crate::diskinfo::{self, DiskUsage};
use crate::util::human_bytes;

/// A finished background job, delivered to the UI thread.
enum Job {
    Scan(Vec<CategoryReport>),
    Clean(Vec<CategoryReport>),
    Large(Vec<LargeFile>),
}

pub struct WCleanApp {
    mode: Mode,
    disk: Option<DiskUsage>,

    /// Selection state, indexed to match [`Category::all`].
    selected: Vec<bool>,
    scan: Vec<CategoryReport>,
    last_freed: Option<u64>,
    notes: Vec<String>,

    large_path: String,
    large_min_mb: u64,
    large_top: usize,
    large: Vec<LargeFile>,

    status: String,
    busy: bool,
    rx: Option<Receiver<Job>>,
    confirm_clean: bool,
}

impl WCleanApp {
    pub fn new() -> Self {
        let cfg = Config::load();

        // Theme precedence: explicit env override > saved preference > dark.
        let mode = match std::env::var("WCLEAN_THEME").ok().as_deref() {
            Some("light") => Mode::Light,
            Some("dark") => Mode::Dark,
            _ => match cfg.theme.as_deref() {
                Some("light") => Mode::Light,
                _ => Mode::Dark,
            },
        };

        // Restore the category selection by stable key; default to all on.
        let selected = match &cfg.selected {
            Some(keys) => Category::all()
                .iter()
                .map(|c| keys.iter().any(|k| k == c.key()))
                .collect(),
            None => vec![true; Category::all().len()],
        };

        Self {
            mode,
            disk: diskinfo::system_drive(),
            selected,
            scan: Vec::new(),
            last_freed: None,
            notes: Vec::new(),
            large_path: cfg.large_path.unwrap_or_else(default_root),
            large_min_mb: cfg.large_min_mb.unwrap_or(100),
            large_top: cfg.large_top.unwrap_or(20),
            large: Vec::new(),
            status: "Ready".to_string(),
            busy: false,
            rx: None,
            confirm_clean: false,
        }
    }

    /// Snapshot the current preferences and persist them to disk.
    fn save_prefs(&self) {
        let selected = Category::all()
            .iter()
            .enumerate()
            .filter(|(i, _)| self.selected[*i])
            .map(|(_, c)| c.key().to_string())
            .collect();
        Config {
            theme: Some(match self.mode {
                Mode::Dark => "dark".into(),
                Mode::Light => "light".into(),
            }),
            selected: Some(selected),
            large_path: Some(self.large_path.clone()),
            large_min_mb: Some(self.large_min_mb),
            large_top: Some(self.large_top),
        }
        .save();
    }

    fn selected_categories(&self) -> Vec<Category> {
        Category::all()
            .iter()
            .enumerate()
            .filter(|(i, _)| self.selected[*i])
            .map(|(_, c)| *c)
            .collect()
    }

    fn scanned_bytes(&self, cat: Category) -> Option<u64> {
        self.scan
            .iter()
            .find(|r| r.category == cat)
            .map(|r| r.stats.bytes)
    }

    /// Bytes reclaimable across the *currently selected* scanned categories.
    fn reclaimable(&self) -> u64 {
        self.selected_categories()
            .iter()
            .filter_map(|c| self.scanned_bytes(*c))
            .sum()
    }

    /// Run `f` on a worker thread; its result arrives via `self.rx`.
    fn spawn<F>(&mut self, ctx: &egui::Context, status: &str, f: F)
    where
        F: FnOnce() -> Job + Send + 'static,
    {
        let (tx, rx) = channel();
        self.rx = Some(rx);
        self.busy = true;
        self.last_freed = None;
        self.status = status.to_string();
        let ctx = ctx.clone();
        thread::spawn(move || {
            let job = f();
            let _ = tx.send(job);
            ctx.request_repaint();
        });
    }

    fn poll(&mut self) {
        let done = self.rx.as_ref().and_then(|rx| rx.try_recv().ok());
        if let Some(job) = done {
            self.busy = false;
            self.rx = None;
            match job {
                Job::Scan(reports) => {
                    self.notes = collect_notes(&reports);
                    self.scan = reports;
                    let total = self.reclaimable();
                    self.status = if total == 0 {
                        "Nothing to clean — you're all tidy.".to_string()
                    } else {
                        format!("Found {} to reclaim.", human_bytes(total))
                    };
                }
                Job::Clean(reports) => {
                    let freed: u64 = reports.iter().map(|r| r.stats.bytes).sum();
                    self.notes = collect_notes(&reports);
                    self.last_freed = Some(freed);
                    self.scan.clear();
                    self.disk = diskinfo::system_drive();
                    self.status = format!("Freed {}.", human_bytes(freed));
                }
                Job::Large(files) => {
                    self.status = format!("Found {} large file(s).", files.len());
                    self.large = files;
                }
            }
        }
    }
}

impl Default for WCleanApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for WCleanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll();
        let p = self.mode.palette();

        self.top_bar(ctx);
        self.status_bar(ctx);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(p.bg)
                    .inner_margin(egui::Margin {
                        left: 22,
                        right: 22,
                        top: 18,
                        bottom: 18,
                    }),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.hero(ui, ctx);
                    ui.add_space(space::LG);
                    self.categories(ui);
                    ui.add_space(space::LG);
                    self.large_section(ui, ctx);
                });
            });

        self.confirm_dialog(ctx);
    }
}

impl WCleanApp {
    fn top_bar(&mut self, ctx: &egui::Context) {
        let p = self.mode.palette();
        egui::TopBottomPanel::top("topbar")
            .frame(
                egui::Frame::default()
                    .fill(p.surface)
                    .inner_margin(egui::Margin::symmetric(22, 12))
                    .stroke(egui::Stroke::new(1.0, p.border)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    widgets::logo(ui, &p, 22.0);
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("wclean")
                            .size(18.0)
                            .strong()
                            .color(p.text_strong),
                    );
                    ui.label(RichText::new("Disk Cleanup").color(p.text_muted));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let icon = if self.mode == Mode::Dark {
                            "Light"
                        } else {
                            "Dark"
                        };
                        if widgets::ghost_button(ui, &p, icon, true).clicked() {
                            self.mode = self.mode.toggled();
                            theme::apply(ctx, self.mode);
                            self.save_prefs();
                        }
                    });
                });
            });
    }

    fn status_bar(&mut self, ctx: &egui::Context) {
        let p = self.mode.palette();
        egui::TopBottomPanel::bottom("statusbar")
            .frame(
                egui::Frame::default()
                    .fill(p.surface)
                    .inner_margin(egui::Margin::symmetric(22, 8))
                    .stroke(egui::Stroke::new(1.0, p.border)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if self.busy {
                        ui.spinner();
                    }
                    ui.label(RichText::new(&self.status).color(p.text));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new("Private · No ads · No telemetry")
                                .small()
                                .color(p.text_muted),
                        );
                    });
                });
            });
    }

    /// The drive-usage hero: ring gauge + headline + primary action.
    fn hero(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let p = self.mode.palette();
        egui::Frame::default()
            .fill(p.surface)
            .stroke(egui::Stroke::new(1.0, p.border))
            .corner_radius(egui::CornerRadius::same(theme::radius::CARD))
            .inner_margin(egui::Margin::same(20))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    widgets::disk_ring(ui, &p, self.disk, 150.0);
                    ui.add_space(space::MD);
                    ui.vertical(|ui| {
                        ui.add_space(space::XS);
                        ui.label(
                            RichText::new("System Drive (C:)")
                                .size(19.0)
                                .strong()
                                .color(p.text_strong),
                        );
                        match self.disk {
                            Some(d) => ui.label(
                                RichText::new(format!(
                                    "{} free of {}",
                                    human_bytes(d.free),
                                    human_bytes(d.total)
                                ))
                                .color(p.text_muted),
                            ),
                            None => ui.label(
                                RichText::new("Drive usage available on Windows")
                                    .color(p.text_muted),
                            ),
                        };
                        ui.add_space(space::SM);

                        if let Some(freed) = self.last_freed {
                            widgets::success_banner(
                                ui,
                                &p,
                                &format!("Freed {} 🎉", human_bytes(freed)),
                            );
                            ui.add_space(space::SM);
                        }

                        self.hero_actions(ui, ctx);
                    });
                });
            });
    }

    fn hero_actions(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let p = self.mode.palette();
        let any_selected = !self.selected_categories().is_empty();
        let scanned = !self.scan.is_empty();
        let reclaimable = self.reclaimable();

        ui.horizontal(|ui| {
            if !scanned {
                if widgets::primary_button(ui, &p, "🔍  Scan", !self.busy && any_selected).clicked()
                {
                    self.start_scan(ctx);
                }
            } else {
                let label = if reclaimable > 0 {
                    format!("Clean up {}", human_bytes(reclaimable))
                } else {
                    "Nothing to clean".to_string()
                };
                if widgets::primary_button(ui, &p, &label, !self.busy && reclaimable > 0).clicked()
                {
                    self.confirm_clean = true;
                }
                if widgets::ghost_button(ui, &p, "Re-scan", !self.busy).clicked() {
                    self.start_scan(ctx);
                }
            }
        });
    }

    fn start_scan(&mut self, ctx: &egui::Context) {
        let cats = self.selected_categories();
        self.spawn(ctx, "Scanning…", move || {
            Job::Scan(cats.iter().map(|&c| cleaner::scan(c)).collect())
        });
    }

    fn categories(&mut self, ui: &mut egui::Ui) {
        let p = self.mode.palette();
        ui.horizontal(|ui| {
            widgets::section_header(
                ui,
                &p,
                "What to clean",
                "Tap a card to include or exclude it.",
            );
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                let all_on = self.selected.iter().all(|&s| s);
                let none_on = self.selected.iter().all(|&s| !s);
                if ui
                    .add_enabled(!none_on, egui::Button::new(RichText::new("Clear").small()))
                    .clicked()
                {
                    self.selected.iter_mut().for_each(|s| *s = false);
                    self.save_prefs();
                }
                if ui
                    .add_enabled(
                        !all_on,
                        egui::Button::new(RichText::new("Select all").small()),
                    )
                    .clicked()
                {
                    self.selected.iter_mut().for_each(|s| *s = true);
                    self.save_prefs();
                }
            });
        });
        ui.add_space(space::XS);

        let bytes: Vec<Option<u64>> = Category::all()
            .iter()
            .map(|c| self.scanned_bytes(*c))
            .collect();

        let mut changed = false;
        for (i, &cat) in Category::all().iter().enumerate() {
            let mut sel = self.selected[i];
            if widgets::category_card(ui, &p, cat, &mut sel, bytes[i]) {
                changed = true;
            }
            self.selected[i] = sel;
            ui.add_space(space::SM);
        }
        if changed {
            self.save_prefs();
        }

        if !self.notes.is_empty() {
            ui.add_space(2.0);
            for note in &self.notes {
                ui.label(
                    RichText::new(format!("• {note}"))
                        .small()
                        .color(p.text_muted),
                );
            }
        }
    }

    fn large_section(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let p = self.mode.palette();
        widgets::section_header(
            ui,
            &p,
            "Find large files",
            "List the biggest files under a folder. Reported only — never deleted.",
        );

        egui::Frame::default()
            .fill(p.surface)
            .stroke(egui::Stroke::new(1.0, p.border))
            .corner_radius(egui::CornerRadius::same(theme::radius::CARD))
            .inner_margin(egui::Margin::same(16))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                let mut opts_changed = false;
                egui::Grid::new("large_opts")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Folder").color(p.text_muted));
                        opts_changed |= ui
                            .add(
                                egui::TextEdit::singleline(&mut self.large_path)
                                    .desired_width(f32::INFINITY),
                            )
                            .changed();
                        ui.end_row();
                        ui.label(RichText::new("Min size (MB)").color(p.text_muted));
                        opts_changed |= ui
                            .add(egui::DragValue::new(&mut self.large_min_mb).range(1..=1_000_000))
                            .changed();
                        ui.end_row();
                        ui.label(RichText::new("Show top").color(p.text_muted));
                        opts_changed |= ui
                            .add(egui::DragValue::new(&mut self.large_top).range(1..=1000))
                            .changed();
                        ui.end_row();
                    });
                if opts_changed {
                    self.save_prefs();
                }

                ui.add_space(space::SM);
                if widgets::primary_button(ui, &p, "📁  Scan folder", !self.busy).clicked() {
                    let path = std::path::PathBuf::from(self.large_path.clone());
                    let min = self.large_min_mb.saturating_mul(1024 * 1024);
                    let top = self.large_top;
                    self.spawn(ctx, "Scanning for large files…", move || {
                        Job::Large(largefiles::scan(&path, min, top))
                    });
                }

                if !self.large.is_empty() {
                    ui.add_space(space::SM);
                    for (i, f) in self.large.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("{:>2}.", i + 1)).color(p.text_muted));
                            ui.label(RichText::new(human_bytes(f.size)).strong().color(p.accent));
                            ui.label(
                                RichText::new(f.path.display().to_string())
                                    .color(p.text)
                                    .small(),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                let reveal = egui::Button::new(
                                    RichText::new("Reveal").small().color(p.accent),
                                )
                                .frame(false);
                                if ui
                                    .add(reveal)
                                    .on_hover_text("Show in file manager")
                                    .clicked()
                                {
                                    reveal_in_file_manager(&f.path);
                                }
                            });
                        });
                    }
                }
            });
    }

    fn confirm_dialog(&mut self, ctx: &egui::Context) {
        if !self.confirm_clean {
            return;
        }
        let p = self.mode.palette();
        let cats = self.selected_categories();
        let total = self.reclaimable();
        let mut open = true;
        egui::Window::new("Confirm cleanup")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .open(&mut open)
            .show(ctx, |ui| {
                ui.set_max_width(360.0);
                ui.label(
                    RichText::new(format!(
                        "This will permanently free {}.",
                        human_bytes(total)
                    ))
                    .color(p.text_strong)
                    .strong(),
                );
                ui.add_space(space::SM);
                for c in &cats {
                    ui.label(RichText::new(format!("{}  {}", c.glyph(), c.title())).color(p.text));
                }
                ui.add_space(space::MD);
                ui.horizontal(|ui| {
                    if widgets::ghost_button(ui, &p, "Cancel", true).clicked() {
                        self.confirm_clean = false;
                    }
                    if widgets::danger_button(ui, &p, "Delete now").clicked() {
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

/// Gather non-empty notes from a set of reports for display under the cards.
fn collect_notes(reports: &[CategoryReport]) -> Vec<String> {
    reports
        .iter()
        .flat_map(|r| r.notes.iter().cloned())
        .collect()
}

/// Open the platform file manager with `path` highlighted (best effort).
fn reveal_in_file_manager(path: &std::path::Path) {
    use std::process::Command;
    let _ = if cfg!(windows) {
        Command::new("explorer")
            .arg(format!("/select,{}", path.display()))
            .spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg("-R").arg(path).spawn()
    } else {
        // Linux/other: open the containing directory.
        let dir = path.parent().unwrap_or(path);
        Command::new("xdg-open").arg(dir).spawn()
    };
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
