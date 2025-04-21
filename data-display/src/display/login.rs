use eframe::egui::Ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct LoginDisplay<'a> {
    logged_in: &'a mut bool, // Receive a mutable reference to a global logged_in var
    show_window: bool,
    failed_attempts: u8,
}

impl<'a> LoginDisplay<'a> {
    pub fn new(logged_in: &'a mut bool) -> Self {
        LoginDisplay {
            logged_in: logged_in,
            show_window: false,
            failed_attempts: 0,
        }
    }

    pub fn draw(&mut self, Ui: Ui) -> () {}

    fn login(&mut self) {
        // Verify With Server...

        // If successful, show sessions window and set fail count to 0
        *self.logged_in = false;

        // If fail, increment fail count
    }

    fn logout(&mut self) {
        // Clear stored user and hide sessions window (should sessions window cascade?)
        *self.logged_in = false;
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
