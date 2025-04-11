// use eframe::egui;

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
}

impl Default for Login {
    fn default() -> Self {
        Self {
            logged_in: false,
            failed_attempts: 0,
        }
    }
}

impl Login {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        //TODO: Load previous app state if any (must enable the `persistence` feature)

        Default::default()
    }

    fn login(&mut self) {
        // Verify With Server...

        // If successful, show sessions window and set fail count to 0
        self.logged_in = true;

        // If fail, increment fail count
    }

    fn logout(&mut self) {
        // Clear stored user and hide sessions window (should sessions window cascade?)
        self.logged_in = false;
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
}

impl eframe::App for Login {
    //TODO: fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::Window::new("Login Manager").show(ctx, |ui| {
            if !self.logged_in {
                Login::show_login_entry(self, ui);
            } else {
                Login::show_logged_in(self, ui);
            }
        });
    }
}
