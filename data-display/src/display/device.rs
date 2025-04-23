//! Device status display and connection settings
//!

use eframe::egui::{Color32, RichText};

/// Displays information about the device and the properties of the connection
///     made to the device.
/// It also provides a connection settings panel.
///
pub struct DeviceDisplay {
    _account_id: String, // Example
}

impl DeviceDisplay {
    pub fn new() -> Self {
        DeviceDisplay {
            _account_id: String::from(""),
        }
    }

    /// Helper function to assign contents for the device window
    fn show_device_info(&mut self, ui: &mut eframe::egui::Ui) -> () {
        // Connection Info Section
        ui.label(RichText::new("Connection").heading().underline());
        ui.label(RichText::new("OFFLINE").color(Color32::RED));
        if ui.link("Settings").clicked() {
            // TODO: Implement
        }
        ui.group(|ui| {
            ui.label("Type:");
            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.label(RichText::new("DIRECT").color(Color32::RED));
                    ui.label("|");
                    ui.label(RichText::new("PROXY").color(Color32::RED));
                });
            });
            ui.label("IPv4: ---");
            ui.label("IPv6: ---");
            ui.label("Cellular Strength: ---");
        });
        ui.add_space(5.0);

        // Hardware info section
        ui.label(RichText::new("Hardware").heading().underline());
        ui.group(|ui| {
            ui.label("CPU Temperature: ---");
            ui.label("CPU Load: ---");
            ui.label("Battery Temperature: ---");
            ui.label("Battery Remaining Capacity: ---");
        });
    }

    /// Performs the draw step for the device window
    pub fn draw(&mut self, ctx: &eframe::egui::Context) -> () {
        eframe::egui::Window::new("Device").show(ctx, |ui| {
            self.show_device_info(ui);
        });
    }
}
