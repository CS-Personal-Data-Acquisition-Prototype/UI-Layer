//! Data recording session manager
//!

use eframe::egui::RichText;

/// Handles displaying and managing historcal data recording sessions and
///     initiates the recording of new data sessions.
///
pub struct SessionDisplay {
    _account_id: String, // Example
}

impl SessionDisplay {
    pub fn new() -> Self {
        SessionDisplay {
            _account_id: String::from(""),
        }
    }

    /// Helper function to assign window contents
    fn show_session_data(&mut self, ui: &mut eframe::egui::Ui) -> () {
        if ui.button("New Session").clicked() {}
        eframe::egui::Grid::new("session_history_grid").show(ui, |ui| {
            ui.label(RichText::new("Timestamp").strong());
            ui.label(RichText::new("Duration").strong());
            ui.label(RichText::new("Actions").strong());
            ui.end_row();

            ui.label("...");
            ui.label("...");
            if ui.link("Download").clicked() {}
            if ui.link("Modify").clicked() {}
            if ui.link("View").clicked() {}
            ui.end_row();
        });
    }

    /// Performs the draw step for the sessions window
    pub fn draw(&mut self, ctx: &eframe::egui::Context) -> () {
        eframe::egui::Window::new("Sessions").show(ctx, |ui| {
            self.show_session_data(ui);
        });
    }
}
