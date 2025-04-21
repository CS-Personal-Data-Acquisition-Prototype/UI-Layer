mod account;
mod device;
mod login;
mod sessions;

use eframe::egui::{self, Color32, RichText};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // <- we don't need this because we're not saving states?
pub struct DisplayApp {
    /* This how you opt-out of serialization of a field
    #[serde(skip)]
    value: f32*/
    // login_name: String, // Usage unimplemented, these are ideas for what to use...
    // login_token: String,
    logged_in: bool,
    failed_attempts: u8,

    window_login: login::LoginDisplay<'a>,

    show_window_sessionhist: bool,
    show_window_account: bool,
    show_window_deviceinfo: bool,
}

impl Default for DisplayApp {
    fn default() -> Self {
        Self {
            logged_in: false,
            failed_attempts: 0,
            show_window_sessionhist: false,
            show_window_account: false,
            show_window_deviceinfo: false,
        }
    }
}

impl DisplayApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_visuals(egui::Visuals {
            ..Default::default()
        });

        //TODO: Load previous app state if any (must enable the `persistence` feature)

        Default::default()
    }

    fn show_session_history(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("New Session").clicked() {}
        egui::Grid::new("session_history_grid").show(ui, |ui| {
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
    fn show_account_info(&mut self, ui: &mut eframe::egui::Ui) {
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
    fn show_device_info(&mut self, ui: &mut eframe::egui::Ui) {
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
}

impl eframe::App for DisplayApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("CS.026 Personal Data Acquision Project 24-25").heading(),
                );
            });
        });

        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            ui.label(
                egui::RichText::new(
                    "Copyright © 2025 CS 46X Personal Data Acquisition Prototype Group ",
                )
                .small(),
            );
        });

        eframe::egui::Window::new("Login Manager").show(ctx, |ui| {
            if !self.logged_in {
                DisplayApp::show_login_entry(self, ui);
            } else {
                DisplayApp::show_logged_in(self, ui);
            }
        });

        if self.show_window_sessionhist {
            eframe::egui::Window::new("Sessions").show(ctx, |ui| {
                DisplayApp::show_session_history(self, ui);
            });
        }

        if self.show_window_sessionhist {
            eframe::egui::Window::new("Account").show(ctx, |ui| {
                DisplayApp::show_account_info(self, ui);
            });
        }

        if self.show_window_sessionhist {
            eframe::egui::Window::new("Device").show(ctx, |ui| {
                DisplayApp::show_device_info(self, ui);
            });
        }
    }

    // TODO:
    // Make a session manager window that's hidden by default.
    // Have main.rs open it on launch
    // Hide/Show it by logging in / out with the login manager window.
}
