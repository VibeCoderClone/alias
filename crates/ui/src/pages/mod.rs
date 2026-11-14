mod editor;
mod home;
mod settings;

use eframe::egui;

#[derive(Default, Clone, PartialEq)]
pub enum Page {
    #[default]
    Home,
    Editor {
        project: usize,
    },
    Settings,
}

impl Page {
    /// Draws the page. Returns Some(page) if it wants to navigate.
    pub fn show(&self, ui: &mut egui::Ui) -> Option<Page> {
        match self {
            Page::Home => home::show(ui),
            Page::Editor { project } => editor::show(ui, *project),
            Page::Settings => settings::show(ui),
        }
    }
}
