//! Account information / settings display and appearance controller
//!

/// Handles functions superficial to the interface
///
/// Including:
/// - Display of account information
/// - Changing account credentials
/// - Changing appearance settings
pub struct AccountDisplay {
    _account_id: String, // Example
}

impl AccountDisplay {
    pub fn new() -> Self {
        AccountDisplay {
            _account_id: String::from(""),
        }
    }

    /// Helper function to assign window contents
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

    /// Performs the draw step for the account window
    pub fn draw(&mut self, ctx: &eframe::egui::Context) -> () {
        eframe::egui::Window::new("Account").show(ctx, |ui| {
            self.show_account_info(ui);
        });
    }
}
