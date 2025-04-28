//! The display manager for the codebase
//!
//! The listed modules are each responsible for a designated section, including: a window, functionalities, and popups for those functionalities.
//!

mod account;
mod device;
mod login;
mod sessions;
mod data;

use eframe::egui;

/// Container for all window-controlling structs. Manages shared resources and calls draw() functions.
pub struct DisplayApp {
    logged_in: Box<bool>,

    window_login: login::LoginDisplay,
    window_account: account::AccountDisplay,
    window_sessions: sessions::SessionDisplay,
    window_device: device::DeviceDisplay,
    window_data: data::DataWindow,
}

impl Default for DisplayApp {
    fn default() -> Self {
        // This allocates a box and unwraps the heap pointer
        let logged_in_ptr: *mut bool = Box::into_raw(Box::new(false));

        Self {
            // This puts the box back together to avoid leaks
            logged_in: unsafe { Box::from_raw(logged_in_ptr) },

            // Declare windows we will draw and provide pointers to shared resources
            window_login: login::LoginDisplay::new(logged_in_ptr),
            window_account: account::AccountDisplay::new(),
            window_sessions: sessions::SessionDisplay::new(),
            window_device: device::DeviceDisplay::new(),
            window_data: data::DataWindow::new(),
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

        Default::default()
    }

    /// Helper function to draw the top EGUI Bar
    fn draw_top_bar(ctx: &egui::Context) -> () {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("CS.026 Personal Data Acquision Project 24-25").heading(),
                );
            });
        });
    }

    /// Helper function to daw the bottom EGUI Bar
    fn draw_bottom_bar(ctx: &egui::Context) -> () {
        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            ui.label(
                egui::RichText::new(
                    "Copyright © 2025 CS 46X Personal Data Acquisition Prototype Group ",
                )
                .small(),
            );
        });
    }
}

/// This Implementation initiates the draw step for everything in the app.
impl eframe::App for DisplayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Draw the top bar
        DisplayApp::draw_top_bar(ctx);

        // Draw the bottom bar
        DisplayApp::draw_bottom_bar(ctx);

        // Draw windows
        self.window_login.draw(ctx);
        if *self.logged_in {
            self.window_account.draw(ctx);
            self.window_sessions.draw(ctx);
            self.window_device.draw(ctx);
            self.window_data.draw(ctx);
        }
    }
}
