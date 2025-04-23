pub struct LoginDisplay {
    logged_in: *mut bool, // Receive a mutable reference to a global logged_in var
    show_window: bool,
    failed_attempts: u8,
}

impl LoginDisplay {
    pub fn new(logged_in: *mut bool) -> Self {
        LoginDisplay {
            logged_in: logged_in,
            show_window: false,
            failed_attempts: 0,
        }
    }

    fn get_logged_in(&self) -> bool {
        return unsafe { *self.logged_in };
    }

    fn login(&mut self) -> () {
        // Verify With Server...

        // If successful, show sessions window and set fail count to 0
        unsafe {
            *self.logged_in = true;
        }

        // If fail, increment fail count
    }

    fn logout(&mut self) -> () {
        // Clear stored user and hide sessions window (should sessions window cascade?)
        unsafe {
            *self.logged_in = false;
        }
    }

    pub fn draw(&mut self, ctx: &eframe::egui::Context) -> () {
        eframe::egui::Window::new("Login Manager").show(ctx, |ui| {
            if !self.get_logged_in() {
                self.show_login_entry(ui);
            } else {
                self.show_logged_in(ui);
            }
        });
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
            self.login();
        }
    }

    fn show_logged_in(&mut self, ui: &mut eframe::egui::Ui) {
        let logout_btn = eframe::egui::Button::new("Logout");

        ui.label("Logged in as: PLACEHOLDER");
        if ui.add(logout_btn).clicked() {
            self.logout();
        }
    }
}
