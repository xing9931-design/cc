//! The wclean graphical interface (egui/eframe).
//!
//! [`run`] launches the window. The UI is split into a design system
//! ([`theme`]), painted components ([`widgets`]) and the screen/state
//! ([`app`]). Long-running work happens off the UI thread.

mod app;
mod theme;
mod widgets;

use eframe::egui;

/// Launch the wclean GUI. Blocks until the window is closed.
pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([720.0, 900.0])
            .with_min_inner_size([520.0, 560.0])
            .with_title("wclean — Disk Cleanup")
            .with_icon(app_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "wclean",
        options,
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx, theme::Mode::from_env());
            Ok(Box::new(app::WCleanApp::new()))
        }),
    )
}

/// Generate the window/taskbar icon in code — the brand mark (accent disc with
/// a white check), so we ship no separate image asset.
fn app_icon() -> egui::IconData {
    const SIZE: usize = 64;
    let mut rgba = vec![0u8; SIZE * SIZE * 4];
    let c = (SIZE as f32 - 1.0) / 2.0;
    let r = SIZE as f32 * 0.46;
    let accent = [0x6Cu8, 0x8C, 0xFF];
    let n = SIZE as f32;

    // Check-mark segments, sized like the in-app logo.
    let a1 = (c - 0.20 * n, c + 0.02 * n);
    let a2 = (c - 0.04 * n, c + 0.16 * n);
    let a3 = (c + 0.22 * n, c - 0.16 * n);
    let half = n * 0.055;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let (fx, fy) = (x as f32, y as f32);
            let d = ((fx - c).powi(2) + (fy - c).powi(2)).sqrt();
            let disc_a = (r - d + 0.5).clamp(0.0, 1.0);
            if disc_a <= 0.0 {
                continue;
            }
            let dist = dist_to_seg((fx, fy), a1, a2).min(dist_to_seg((fx, fy), a2, a3));
            let check_a = (half - dist + 0.5).clamp(0.0, 1.0);
            let idx = (y * SIZE + x) * 4;
            for k in 0..3 {
                rgba[idx + k] =
                    (accent[k] as f32 * (1.0 - check_a) + 255.0 * check_a).round() as u8;
            }
            rgba[idx + 3] = (disc_a * 255.0).round() as u8;
        }
    }

    egui::IconData {
        rgba,
        width: SIZE as u32,
        height: SIZE as u32,
    }
}

/// Euclidean distance from point `p` to the segment `a`–`b`.
fn dist_to_seg(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (px, py) = p;
    let (ax, ay) = a;
    let (bx, by) = b;
    let (dx, dy) = (bx - ax, by - ay);
    let len2 = dx * dx + dy * dy;
    let t = if len2 == 0.0 {
        0.0
    } else {
        (((px - ax) * dx + (py - ay) * dy) / len2).clamp(0.0, 1.0)
    };
    let (cx, cy) = (ax + t * dx, ay + t * dy);
    ((px - cx).powi(2) + (py - cy).powi(2)).sqrt()
}
