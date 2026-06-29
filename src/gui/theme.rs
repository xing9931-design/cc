//! The wclean design system: color tokens, spacing, radii and typography.
//!
//! A single [`Palette`] drives the whole app so the visual language stays
//! consistent. [`apply`] translates the tokens into an egui `Style`.

use eframe::egui::{self, Color32, CornerRadius, FontFamily, FontId, TextStyle};

/// Named colors for one theme (dark or light).
#[derive(Clone, Copy)]
pub struct Palette {
    pub bg: Color32,
    pub surface: Color32,
    pub surface_alt: Color32,
    pub border: Color32,
    pub text_strong: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_weak: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub danger: Color32,
}

/// Corner radii (px).
pub mod radius {
    pub const CARD: u8 = 14;
    pub const CONTROL: u8 = 10;
    /// Fully rounded (capsule) — egui clamps to half the height.
    pub const PILL: u8 = 255;
}

/// Spacing scale (px) — an 8pt-ish rhythm.
pub mod space {
    pub const XS: f32 = 6.0;
    pub const SM: f32 = 10.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 24.0;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dark,
    Light,
}

impl Mode {
    pub fn toggled(self) -> Mode {
        match self {
            Mode::Dark => Mode::Light,
            Mode::Light => Mode::Dark,
        }
    }

    /// Initial mode, honoring `WCLEAN_THEME=light|dark` (defaults to dark).
    pub fn from_env() -> Mode {
        match std::env::var("WCLEAN_THEME").as_deref() {
            Ok("light") => Mode::Light,
            _ => Mode::Dark,
        }
    }

    pub fn palette(self) -> Palette {
        match self {
            Mode::Dark => Palette::dark(),
            Mode::Light => Palette::light(),
        }
    }
}

impl Palette {
    pub fn dark() -> Self {
        Self {
            bg: Color32::from_rgb(0x0F, 0x11, 0x15),
            surface: Color32::from_rgb(0x17, 0x1A, 0x21),
            surface_alt: Color32::from_rgb(0x1E, 0x22, 0x2B),
            border: Color32::from_rgb(0x2A, 0x2F, 0x3A),
            text_strong: Color32::from_rgb(0xEC, 0xEE, 0xF2),
            text: Color32::from_rgb(0xC3, 0xC8, 0xD2),
            text_muted: Color32::from_rgb(0x8A, 0x90, 0x9C),
            accent: Color32::from_rgb(0x6C, 0x8C, 0xFF),
            accent_weak: Color32::from_rgb(0x2A, 0x33, 0x55),
            success: Color32::from_rgb(0x34, 0xD3, 0x99),
            warning: Color32::from_rgb(0xF5, 0xB7, 0x4B),
            danger: Color32::from_rgb(0xF8, 0x71, 0x71),
        }
    }

    pub fn light() -> Self {
        Self {
            bg: Color32::from_rgb(0xF5, 0xF6, 0xF8),
            surface: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            surface_alt: Color32::from_rgb(0xEE, 0xF1, 0xF5),
            border: Color32::from_rgb(0xDF, 0xE3, 0xEA),
            text_strong: Color32::from_rgb(0x16, 0x1A, 0x20),
            text: Color32::from_rgb(0x39, 0x40, 0x4B),
            text_muted: Color32::from_rgb(0x6B, 0x72, 0x80),
            accent: Color32::from_rgb(0x3B, 0x6F, 0xF5),
            accent_weak: Color32::from_rgb(0xDD, 0xE6, 0xFE),
            success: Color32::from_rgb(0x0E, 0x9F, 0x6E),
            warning: Color32::from_rgb(0xC2, 0x7A, 0x0E),
            danger: Color32::from_rgb(0xDC, 0x35, 0x35),
        }
    }
}

/// Apply a palette to the egui context: visuals, spacing and type scale.
pub fn apply(ctx: &egui::Context, mode: Mode) {
    let p = mode.palette();
    let mut style = (*ctx.style()).clone();

    // Typography — a clear, restrained scale.
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(24.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(15.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(12.5, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(13.5, FontFamily::Monospace),
        ),
    ]
    .into();

    // Spacing — generous, calm.
    style.spacing.item_spacing = egui::vec2(space::SM, space::SM);
    style.spacing.button_padding = egui::vec2(16.0, 9.0);
    style.spacing.menu_margin = egui::Margin::same(8);
    style.spacing.window_margin = egui::Margin::same(0);
    style.spacing.interact_size.y = 32.0;

    let mut v = if mode == Mode::Dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    v.override_text_color = Some(p.text);
    v.panel_fill = p.bg;
    v.window_fill = p.surface;
    v.extreme_bg_color = p.surface_alt;
    v.faint_bg_color = p.surface_alt;
    v.window_stroke = egui::Stroke::new(1.0, p.border);
    v.window_corner_radius = CornerRadius::same(radius::CARD);
    v.selection.bg_fill = p.accent_weak;
    v.selection.stroke = egui::Stroke::new(1.0, p.accent);
    v.hyperlink_color = p.accent;

    let r = CornerRadius::same(radius::CONTROL);
    // Non-interactive (labels, separators).
    v.widgets.noninteractive.bg_fill = p.surface;
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, p.text);
    v.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, p.border);
    v.widgets.noninteractive.corner_radius = r;
    // Inactive controls (buttons, checkboxes at rest).
    v.widgets.inactive.bg_fill = p.surface_alt;
    v.widgets.inactive.weak_bg_fill = p.surface_alt;
    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, p.text);
    v.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, p.border);
    v.widgets.inactive.corner_radius = r;
    // Hovered.
    v.widgets.hovered.bg_fill = p.accent_weak;
    v.widgets.hovered.weak_bg_fill = p.accent_weak;
    v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, p.text_strong);
    v.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, p.accent);
    v.widgets.hovered.corner_radius = r;
    // Active / pressed.
    v.widgets.active.bg_fill = p.accent;
    v.widgets.active.weak_bg_fill = p.accent;
    v.widgets.active.fg_stroke = egui::Stroke::new(1.0, Color32::WHITE);
    v.widgets.active.bg_stroke = egui::Stroke::new(1.0, p.accent);
    v.widgets.active.corner_radius = r;

    style.visuals = v;
    ctx.set_style(style);
}
