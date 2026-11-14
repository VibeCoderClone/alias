use super::Page;
use crate::theme;
use eframe::egui::{self, RichText};

pub fn show(ui: &mut egui::Ui) -> Option<Page> {
    let mut next = None;

    if ui.button("← Back").clicked() {
        next = Some(Page::Home);
    }
    ui.add_space(12.0);
    ui.label(RichText::new("Settings").size(24.0).color(theme::TEXT));

    next
}
