mod gradient;
mod pages;
mod theme;

use eframe::egui;
use pages::Page;
use std::sync::Arc;

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "thai".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/font/Outfit/static/Outfit-Regular.ttf"
        ))),
    );
    // Add as a fallback after the default fonts
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .push("thai".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("thai".to_owned());
    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([390.0, 844.0]), // phone-sized window
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Alias",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    page: Page,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Each page returns Some(next_page) when it wants to navigate
        let next = egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::same(16)),
            )
            .show(ui, |ui| self.page.show(ui))
            .inner;

        if let Some(page) = next {
            self.page = page;
        }

        // Floating FPS overlay, shown on every page
        egui::Area::new(egui::Id::new("floating_toolbar"))
            .order(egui::Order::Debug)
            .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, -12.0])
            .show(ui.ctx(), |ui| {
                let dt = ui.input(|i| i.stable_dt);
                ui.label("Another generic video editor v0.1.1");
                ui.label(format!("FPS: {:.0}", 1.0 / dt));
            });
    }
}
