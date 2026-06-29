//! Reusable, painted UI components built on the design tokens in [`super::theme`].

use std::f32::consts::PI;

use eframe::egui::{
    self, Align, Color32, CornerRadius, FontId, Layout, Margin, Pos2, RichText, Sense, Stroke, Ui,
    Vec2,
};

use super::theme::{radius, space, Palette};
use crate::cleaner::{Category, Risk};
use crate::diskinfo::DiskUsage;
use crate::util::human_bytes;

/// A circular disk-usage gauge with the percentage in the middle.
///
/// The fill sweeps in and the percentage counts up on first paint and whenever
/// the value changes, via egui's animation curves.
pub fn disk_ring(ui: &mut Ui, p: &Palette, usage: Option<DiskUsage>, diameter: f32) {
    let target = usage.map(|u| u.used_fraction()).unwrap_or(0.0);
    // Ease the displayed fraction toward the target (auto-requests repaints).
    let fraction = ui
        .ctx()
        .animate_value_with_time(egui::Id::new("wclean_disk_ring"), target, 0.8);

    let (rect, _) = ui.allocate_exact_size(Vec2::splat(diameter), Sense::hover());
    let painter = ui.painter_at(rect);
    let center = rect.center();
    let radius = diameter * 0.5 - 8.0;
    let thickness = diameter * 0.085;

    // Track.
    painter.add(egui::Shape::line(
        arc_points(center, radius, -PI / 2.0, -PI / 2.0 + 2.0 * PI, 96),
        Stroke::new(thickness, p.surface_alt),
    ));

    let value_color = match fraction {
        f if f >= 0.9 => p.danger,
        f if f >= 0.75 => p.warning,
        _ => p.accent,
    };

    if fraction > 0.0 {
        let start = -PI / 2.0;
        let end = start + 2.0 * PI * fraction.clamp(0.0, 1.0);
        let pts = arc_points(center, radius, start, end, 96);
        // Rounded caps faked with end dots.
        if let (Some(first), Some(last)) = (pts.first(), pts.last()) {
            painter.circle_filled(*first, thickness * 0.5, value_color);
            painter.circle_filled(*last, thickness * 0.5, value_color);
        }
        painter.add(egui::Shape::line(pts, Stroke::new(thickness, value_color)));
    }

    // Center text.
    match usage {
        Some(_) => {
            painter.text(
                center - Vec2::new(0.0, 8.0),
                egui::Align2::CENTER_CENTER,
                format!("{}%", (fraction * 100.0).round() as u32),
                FontId::proportional(diameter * 0.24),
                p.text_strong,
            );
            painter.text(
                center + Vec2::new(0.0, diameter * 0.16),
                egui::Align2::CENTER_CENTER,
                "in use",
                FontId::proportional(diameter * 0.085),
                p.text_muted,
            );
        }
        None => {
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                "C:",
                FontId::proportional(diameter * 0.2),
                p.text_muted,
            );
        }
    }
}

/// Points along a circular arc, for stroking with [`egui::Shape::line`].
fn arc_points(center: Pos2, radius: f32, start: f32, end: f32, segments: usize) -> Vec<Pos2> {
    (0..=segments)
        .map(|i| {
            let t = i as f32 / segments as f32;
            let a = start + (end - start) * t;
            center + Vec2::new(a.cos(), a.sin()) * radius
        })
        .collect()
}

/// A selectable category card. Returns `true` if its selection changed.
pub fn category_card(
    ui: &mut Ui,
    p: &Palette,
    cat: Category,
    selected: &mut bool,
    scanned_bytes: Option<u64>,
) -> bool {
    let fill = if *selected { p.accent_weak } else { p.surface };
    let stroke = if *selected {
        Stroke::new(1.5, p.accent)
    } else {
        Stroke::new(1.0, p.border)
    };

    let inner = egui::Frame::default()
        .fill(fill)
        .stroke(stroke)
        .corner_radius(CornerRadius::same(radius::CARD))
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(RichText::new(cat.glyph()).size(24.0));
                ui.add_space(space::XS);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(cat.title())
                            .strong()
                            .size(15.5)
                            .color(p.text_strong),
                    );
                    ui.label(RichText::new(cat.description()).small().color(p.text_muted));
                    ui.add_space(2.0);
                    risk_badge(ui, p, cat.risk());
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    selection_dot(ui, p, *selected);
                    if let Some(bytes) = scanned_bytes {
                        ui.label(
                            RichText::new(human_bytes(bytes))
                                .strong()
                                .size(15.0)
                                .color(p.success),
                        );
                    }
                });
            });
        });

    let resp = inner.response.interact(Sense::click());
    if resp.clicked() {
        *selected = !*selected;
        true
    } else {
        false
    }
}

/// A filled circle that reads as a checkbox for the card.
fn selection_dot(ui: &mut Ui, p: &Palette, selected: bool) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
    let painter = ui.painter_at(rect);
    let c = rect.center();
    if selected {
        painter.circle_filled(c, 10.0, p.accent);
        // Check mark.
        let s = Stroke::new(2.0, Color32::WHITE);
        painter.line_segment([c + Vec2::new(-4.0, 0.5), c + Vec2::new(-1.0, 3.5)], s);
        painter.line_segment([c + Vec2::new(-1.0, 3.5), c + Vec2::new(4.5, -3.0)], s);
    } else {
        painter.circle_stroke(c, 9.5, Stroke::new(1.5, p.border));
    }
}

/// A small pill conveying how safe a category is to clean.
pub fn risk_badge(ui: &mut Ui, p: &Palette, risk: Risk) {
    let (fg, bg) = match risk {
        Risk::Safe => (p.success, p.success.linear_multiply(0.18)),
        Risk::Mild => (p.warning, p.warning.linear_multiply(0.18)),
    };
    pill(ui, risk.label(), fg, bg);
}

/// The wclean app mark: a rounded accent square with a clean sweep.
pub fn logo(ui: &mut Ui, p: &Palette, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, CornerRadius::same(6), p.accent);
    let c = rect.center();
    let s = Stroke::new(2.2, Color32::WHITE);
    // A small check/sweep mark.
    painter.line_segment(
        [
            c + Vec2::new(-size * 0.22, 0.0),
            c + Vec2::new(-size * 0.05, size * 0.18),
        ],
        s,
    );
    painter.line_segment(
        [
            c + Vec2::new(-size * 0.05, size * 0.18),
            c + Vec2::new(size * 0.24, -size * 0.2),
        ],
        s,
    );
}

/// A rounded label chip.
pub fn pill(ui: &mut Ui, text: &str, fg: Color32, bg: Color32) {
    egui::Frame::default()
        .fill(bg)
        .corner_radius(CornerRadius::same(radius::PILL))
        .inner_margin(Margin::symmetric(8, 3))
        .show(ui, |ui| {
            ui.label(RichText::new(text).small().color(fg));
        });
}

/// One row of a storage breakdown: a name, a proportional bar, and a size.
///
/// `fraction` (0..=1) sizes the filled portion of the bar; `accent` paints it in
/// the accent color (the largest / loose-files row). `clickable` rows read as a
/// drill-down link (accent name + pointer cursor). Returns the row response.
pub fn usage_row(
    ui: &mut Ui,
    p: &Palette,
    name: &str,
    size_text: &str,
    fraction: f32,
    accent: bool,
    clickable: bool,
) -> egui::Response {
    let inner = ui.horizontal(|ui| {
        ui.add_sized(
            [150.0, 18.0],
            egui::Label::new(RichText::new(name).color(p.text_strong))
                .truncate()
                .selectable(false),
        );
        // Reserve room for the size label, then fill the rest with the bar.
        let bar_w = (ui.available_width() - 86.0).max(48.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::new(bar_w, 12.0), Sense::hover());
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, CornerRadius::same(4), p.surface_alt);
        let mut fill = rect;
        fill.set_width(rect.width() * fraction.clamp(0.0, 1.0));
        let color = if accent { p.accent } else { p.text_muted };
        painter.rect_filled(fill, CornerRadius::same(4), color);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(size_text).small().color(p.text_muted));
        });
    });

    let resp = inner.response.interact(Sense::click());
    if clickable {
        resp.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        resp
    }
}

/// A prominent filled action button.
pub fn primary_button(ui: &mut Ui, p: &Palette, text: &str, enabled: bool) -> egui::Response {
    let btn = egui::Button::new(RichText::new(text).color(Color32::WHITE).strong())
        .fill(if enabled { p.accent } else { p.surface_alt })
        .corner_radius(CornerRadius::same(radius::CONTROL))
        .min_size(Vec2::new(0.0, 38.0));
    ui.add_enabled(enabled, btn)
}

/// A destructive filled action button (used in the confirm dialog).
pub fn danger_button(ui: &mut Ui, p: &Palette, text: &str) -> egui::Response {
    let btn = egui::Button::new(RichText::new(text).color(Color32::WHITE).strong())
        .fill(p.danger)
        .corner_radius(CornerRadius::same(radius::CONTROL))
        .min_size(Vec2::new(0.0, 34.0));
    ui.add(btn)
}

/// A subtle secondary (outline) button.
pub fn ghost_button(ui: &mut Ui, p: &Palette, text: &str, enabled: bool) -> egui::Response {
    let btn = egui::Button::new(RichText::new(text).color(p.text))
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::new(1.0, p.border))
        .corner_radius(CornerRadius::same(radius::CONTROL))
        .min_size(Vec2::new(0.0, 38.0));
    ui.add_enabled(enabled, btn)
}

/// A section title with an optional muted subtitle.
pub fn section_header(ui: &mut Ui, p: &Palette, title: &str, subtitle: &str) {
    ui.label(
        RichText::new(title)
            .size(17.0)
            .strong()
            .color(p.text_strong),
    );
    if !subtitle.is_empty() {
        ui.label(RichText::new(subtitle).small().color(p.text_muted));
    }
    ui.add_space(space::XS);
}

/// A full-width success banner shown after a clean.
pub fn success_banner(ui: &mut Ui, p: &Palette, text: &str) {
    egui::Frame::default()
        .fill(p.success.linear_multiply(0.16))
        .stroke(Stroke::new(1.0, p.success.linear_multiply(0.5)))
        .corner_radius(CornerRadius::same(radius::CONTROL))
        .inner_margin(Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(RichText::new("✔").color(p.success).strong());
                ui.label(RichText::new(text).color(p.text_strong).strong());
            });
        });
}
