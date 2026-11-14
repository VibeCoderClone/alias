use super::Page;
use crate::theme;
use eframe::egui::{self, RichText};

pub fn show(ui: &mut egui::Ui) -> Option<Page> {
    let mut next = None;

    let chevron = egui::Image::new(egui::include_image!(
        "../../assets/icon/chevron-left-solid-white.png"
    ))
    .fit_to_exact_size(egui::vec2(24.0, 24.0))
    .tint(theme::TEXT);

    if ui
        .add(egui::Button::image(chevron).frame(false))
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text("Back")
        .clicked()
    {
        next = Some(Page::Home);
    }

    ui.add_space(12.0);
    ui.label(RichText::new("Settings").size(24.0).color(theme::TEXT));

    next
}
