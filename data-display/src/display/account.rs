pub struct AccountDisplay {
    _account_id: String, // Example
}

impl AccountDisplay {
    pub fn new() -> Self {
        AccountDisplay {
            _account_id: String::from(""),
        }
    }

    fn show_account_info(&mut self, ui: &mut eframe::egui::Ui) -> () {
        ui.label("User: PLACEHOLDER");
        ui.label("Email: EXAMPLE@DOMAIN.COM");
        if ui.link("Change Password").clicked() {
            // TODO: Implement
        }
        if ui.link("Regenerate Device Keys").clicked() {
            // TODO: Implement
        }
        if ui.link("Appearance Settings").clicked() {

            // TODO: Implement
        }
    }

    pub fn draw(&mut self, ctx: &eframe::egui::Context) -> () {
        eframe::egui::Window::new("Account").show(ctx, |ui| {
            self.show_account_info(ui);
        });
    }
}
