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
            .with_title("wclean — Disk Cleanup"),
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
