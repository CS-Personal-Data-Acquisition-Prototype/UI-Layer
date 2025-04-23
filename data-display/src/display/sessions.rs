use eframe::egui::RichText;

pub struct SessionDisplay {
    _account_id: String, // Example
}

impl SessionDisplay {
    pub fn new() -> Self {
        SessionDisplay {
            _account_id: String::from(""),
        }
    }

    fn show_session_history(&mut self, ui: &mut eframe::egui::Ui) -> () {
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

    pub fn draw(&mut self, ctx: &eframe::egui::Context) -> () {
        eframe::egui::Window::new("Sessions").show(ctx, |ui| {
            self.show_session_history(ui);
        });
    }
}
