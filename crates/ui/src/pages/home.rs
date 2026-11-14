use super::Page;
use eframe::egui;

mod desktop;
mod mobile;

/// Below this width we use mobile. Tablets (>= 600) get desktop.
const DESKTOP_BREAKPOINT: f32 = 600.0;

pub fn show(ui: &mut egui::Ui) -> Option<Page> {
    if ui.available_width() < DESKTOP_BREAKPOINT {
        mobile::show(ui)
    } else {
        desktop::show(ui)
    }
}
