//! wclean-gui — graphical front-end. The implementation lives in the library
//! (`wclean::gui`) so it can be tested and reused; this binary is a thin shim.

#![cfg_attr(windows, windows_subsystem = "windows")]

fn main() -> eframe::Result<()> {
    wclean::gui::run()
}
