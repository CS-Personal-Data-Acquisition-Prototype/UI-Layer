use eframe::egui::{self, Color32, RichText};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Login {
    /* This how you opt-out of serialization of a field
    #[serde(skip)]
    value: f32*/
    // login_name: String, // Usage unimplemented, these are ideas for what to use...
    // login_token: String,
    logged_in: bool,
    failed_attempts: u8,
    show_window_sessionhist: bool,
    show_window_account: bool,
    show_window_deviceinfo: bool,
}

impl Default for Login {
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

impl Login {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_visuals(egui::Visuals {
            ..Default::default()
        });

        //TODO: Load previous app state if any (must enable the `persistence` feature)

        Default::default()
    }

    fn login(&mut self) {
        // Verify With Server...

        // If successful, show sessions window and set fail count to 0
        self.logged_in = true;
        self.show_window_sessionhist = true;
        self.show_window_account = true;
        self.show_window_deviceinfo = true;

        // If fail, increment fail count
    }

    fn logout(&mut self) {
        // Clear stored user and hide sessions window (should sessions window cascade?)
        self.logged_in = false;
        self.show_window_sessionhist = false;
        self.show_window_account = false;
        self.show_window_deviceinfo = false;
    }

    fn show_login_entry(&mut self, ui: &mut eframe::egui::Ui) {
        let mut username_str: String = String::from("");
        let mut passwd_str: String = String::from("");

        let username_widget = eframe::egui::TextEdit::singleline(&mut username_str);
        let passwd_widget = eframe::egui::TextEdit::singleline(&mut passwd_str);

        let login_btn = eframe::egui::Button::new("Login");

        // TODO: Show failed login attempt

        ui.label("Username:");
        ui.add(username_widget);

        ui.label("Password:");
        ui.add(passwd_widget);

        if ui.add(login_btn).clicked() {
            Login::login(self);
        }
    }

    fn show_logged_in(&mut self, ui: &mut eframe::egui::Ui) {
        let logout_btn = eframe::egui::Button::new("Logout");

        ui.label("Logged in as: PLACEHOLDER");
        if ui.add(logout_btn).clicked() {
            Login::logout(self);
        }
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

impl eframe::App for Login {
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
                Login::show_login_entry(self, ui);
            } else {
                Login::show_logged_in(self, ui);
            }
        });

        if self.show_window_sessionhist {
            eframe::egui::Window::new("Sessions").show(ctx, |ui| {
                Login::show_session_history(self, ui);
            });
        }

        if self.show_window_sessionhist {
            eframe::egui::Window::new("Account").show(ctx, |ui| {
                Login::show_account_info(self, ui);
            });
        }

        if self.show_window_sessionhist {
            eframe::egui::Window::new("Device").show(ctx, |ui| {
                Login::show_device_info(self, ui);
            });
        }
    }

    // TODO:
    // Make a session manager window that's hidden by default.
    // Have main.rs open it on launch
    // Hide/Show it by logging in / out with the login manager window.
}
