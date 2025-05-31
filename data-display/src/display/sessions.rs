//! Data recording session manager
//!

extern crate client;
use client::api::{session};

//use eframe::egui::RichText;

use serde::Deserialize;

/// Handles displaying and managing historcal data recording sessions and
///     initiates the recording of new data sessions.
///

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Row {
    pub session_id: i64,
    pub username: String, 
}

#[derive(Deserialize)]
pub struct SessionResponse {
    pub sessions: Vec<Row>,
}

#[allow(dead_code)]
pub struct SessionDisplay {
    username: *mut String, 
    session_id_str: String,
    sessions: Vec<Row>,
    loaded: bool,
    current_session: *mut String,
}

impl SessionDisplay {
    pub fn new(username: *mut String, current_session: *mut String) -> Self {  
        SessionDisplay {
            username,
            session_id_str: String::from(""),
            sessions: Vec::new(),
            loaded: false,
            current_session,
        }
    }

    /// Helper function to assign window contents
    fn show_session_data(&mut self, ui: &mut eframe::egui::Ui) -> () {
        let account_id = unsafe {&*self.username};

        // unused in temp solution
        //let session_id_widget = eframe::egui::TextEdit::singleline(&mut self.session_id_str);
        //ui.add(session_id_widget);
        
        if ui.button("New Session").clicked() {
            // unused in temp solution
            //let id = self.session_id_str.trim().to_string();
            let client = client::get_client();

            wasm_bindgen_futures::spawn_local(async move {
                let (status, _val) = session::create_session(&client, account_id).await;
            
                if status == 201 {
                    web_sys::console::log_1( &format!("New session success. Status: {}", status).into() );
                } else {
                    web_sys::console::log_1( &format!("New session failed. Status: {}", status).into() );
                }
            });

            self.loaded = false;
        }
        
        if !self.loaded {
            self.load_sessions()
        }

        for row in &self.sessions {
            ui.horizontal(|ui| {
                ui.label(&row.session_id.to_string());
                if ui.link("Download").clicked() {}
                if ui.link("Modify").clicked() {}
                if ui.link("View").clicked() {
                    unsafe {
                        *self.current_session = row.session_id.to_string();
                    }
                }
            });
        }
    }

    fn load_sessions(&mut self) {
        self.loaded = true;
        let sessions_ptr: *mut Vec<Row> = &mut self.sessions;
        let account_id = unsafe { &*self.username }.clone();
        let client = client::get_client();

        wasm_bindgen_futures::spawn_local(async move {
            let (status, val) = session::view_sessions_by_user(&client, &account_id).await;
        
            if status == 200 {
                web_sys::console::log_1( &format!("Session fetch sucess. Status: {}", status).into() );

                if let Some(val) = val {
                    match serde_json::from_value::<SessionResponse>(val) {
                        Ok(parsed) => {
                            unsafe {
                                *sessions_ptr = parsed.sessions;
                            }
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("Failed to parse sessions: {}", e).into());
                        }
                    }
                }  
            } else {
                web_sys::console::log_1( &format!("Session fetch failed. Status: {}", status).into() );
            }
        });
    }

    /// Performs the draw step for the sessions window
    pub fn draw(&mut self, ctx: &eframe::egui::Context) -> () {
        eframe::egui::Window::new("Sessions").show(ctx, |ui| {
            self.show_session_data(ui);
        });
    }
}
