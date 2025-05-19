//! The login manager

extern crate client;
//use client::api::{auth};
use client::api::{user};

/// Handles drawing the login window and facilitates authentication with the backend api.
pub struct LoginDisplay {
    /// Pointer received at creation from the display manager to control
    ///     window drawing based on login state.
    logged_in: *mut bool,
    username: *mut String,

    /// Tracker for failed login attempts
    _failed_attempts: u8, // unimplemented

    /// Username field passed into a widget
    username_str: String,
    /// Password field passed into a widget
    password_str: String,
}

impl LoginDisplay {
    pub fn new(logged_in: *mut bool, username: *mut String) -> Self {
        LoginDisplay {
            logged_in: logged_in,
            username: username,
            _failed_attempts: 0,

            username_str: String::from(""),
            password_str: String::from(""),
        }
    }

    /// Wrapper for the unsafe operation to dereference the self.logged_in property
    fn get_logged_in(&self) -> bool {
        return unsafe { *self.logged_in };
    }

    /// Attempt to authenticate with the server.
    ///
    /// If successful, notify the rest of the client to update and
    ///     enable drawing of other windows.
    ///
    /// If failed, noify user and track number of failed attempts.
    fn login(&mut self) -> () {
        // Verify With Server...

        // If successful, show sessions window and set fail count to 0
        unsafe {
            *self.logged_in = true;
            *self.username = self.username_str.trim().to_string();
        }

        // If fail, increment fail count
    }

    /// Clear the client state and disable drawing of other windows
    fn logout(&mut self) -> () {
        // Clear stored user and hide sessions window (should sessions window cascade?)
        unsafe {
            *self.logged_in = false;
        }
    }

    /// Helper function to draw window contents when we are not logged in
    fn show_login_entry(&mut self, ui: &mut eframe::egui::Ui) {
        let username_widget = eframe::egui::TextEdit::singleline(&mut self.username_str);
        let passwd_widget = eframe::egui::TextEdit::singleline(&mut self.password_str);

        let login_btn = eframe::egui::Button::new("Login");
        let new_btn = eframe::egui::Button::new("Create New User");

        // TODO: Show failed login attempt

        ui.label("Username:");
        ui.add(username_widget);

        ui.label("Password:");
        ui.add(passwd_widget);

        if ui.add(new_btn).clicked() {
            let username = self.username_str.trim().to_string();
            let password = self.password_str.trim().to_string();

            let client = client::get_client();

            wasm_bindgen_futures::spawn_local(async move {
                let (status, _val) = user::create_user(&client, &username, &password,).await;
            
                if status == 201 {
                    web_sys::console::log_1( &format!("Success. Status: {}", status).into() );
                } else {
                    web_sys::console::log_1( &format!("Failed. Status: {}", status).into() );
                }
            });
        };

        // authentication currently bypassed
        if ui.add(login_btn).clicked() {
            // let username = self.username_str.trim().to_string();
            // let password = self.password_str.trim().to_string();

            // authentication is currently broken

            // let client = client::get_client();
            
            // wasm_bindgen_futures::spawn_local(async move {
            //     let (status, _val, _val2) = auth::user_login(&client, &username, &password,).await;
            
            //     if status == 201 {
            //         web_sys::console::log_1( &format!("Success. Status: {}", status).into() );
            //     } else {
            //         web_sys::console::log_1( &format!("Failed. Status: {}", status).into() );
            //     }
            // });
            
            self.login();
        };
    }

    /// Helper function to draw window contents when we are logged in
    fn show_logged_in(&mut self, ui: &mut eframe::egui::Ui) {
        let logout_btn = eframe::egui::Button::new("Logout");

        unsafe {
            ui.label( format!("Logged in as: {}", *self.username));
        }

        if ui.add(logout_btn).clicked() {
            self.logout();
        }
    }

    /// Performs the draw step for the login window
    pub fn draw(&mut self, ctx: &eframe::egui::Context) -> () {
        eframe::egui::Window::new("Login Manager").show(ctx, |ui| {
            if !self.get_logged_in() {
                self.show_login_entry(ui);
            } else {
                self.show_logged_in(ui);
            }
        });
    }
}
