use super::Page;
use crate::{gradient::rounded_gradient_shape, theme};
use eframe::egui::{self, Color32, RichText, Shape};

pub fn show(ui: &mut egui::Ui) -> Option<Page> {
    let mut next = None;

    ui.horizontal(|ui| {
        ui.label(RichText::new("Alias").size(24.0).color(theme::TEXT));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let gear = egui::Image::new(egui::include_image!(
                "../../../assets/icon/gear-solid-white.png"
            ))
            .fit_to_exact_size(egui::vec2(24.0, 24.0))
            .tint(theme::TEXT);

            if ui
                .add(egui::Button::image(gear).frame(false))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text("Settings")
                .clicked()
            {
                next = Some(Page::Settings);
            }
        });
    });

    ui.add_space(12.0);

    if new_project_card(ui) {
        next = Some(Page::Editor { project: 0 });
    }

    ui.add_space(20.0);
    ui.label(
        RichText::new("Your Previous Projects")
            .size(18.0)
            .color(theme::TEXT),
    );

    egui::ScrollArea::vertical().show(ui, |_ui| {
        // project rows go here
    });

    next
}

/// Returns true when the card (anywhere on the gradient) was clicked.
fn new_project_card(ui: &mut egui::Ui) -> bool {
    let bg_slot = ui.painter().add(Shape::Noop);

    let res = egui::Frame::new()
        .corner_radius(42)
        .show(ui, |ui| {
            ui.set_max_size(egui::vec2(233.0, 142.0));
            ui.set_min_size(egui::vec2(233.0, 142.0));

            // label (18pt text, ~22px tall) + gap + circle
            let content_h = 22.0 + ui.spacing().item_spacing.y + 42.0;
            ui.add_space((142.0 - content_h) / 2.0);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(RichText::new("New Project").size(18.0).color(theme::TEXT));

                // circle is just decoration now, the whole card handles the click
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(42.0, 42.0), egui::Sense::hover());

                ui.painter().circle_filled(rect.center(), 21.0, theme::TEXT);
                let icon_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(24.0, 24.0));

                ui.put(
                    icon_rect,
                    egui::Image::new(egui::include_image!("../../../assets/icon/plus-solid.png")),
                );
            });
        })
        .response;

    ui.painter().set(
        bg_slot,
        rounded_gradient_shape(
            res.rect,
            42.0,
            20.0,
            1.4,
            Color32::from_rgb(68, 56, 80),
            Color32::from_rgb(161, 136, 166),
        ),
    );

    // make the whole card clickable
    ui.interact(
        res.rect,
        ui.id().with("new_project_card"),
        egui::Sense::click(),
    )
    .on_hover_cursor(egui::CursorIcon::PointingHand)
    .clicked()
}
