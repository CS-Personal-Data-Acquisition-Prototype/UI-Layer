//! Main data display window
//!

extern crate client;
use client::api::{session_sensor_data};

use eframe::egui::{ComboBox, Frame};
use egui_extras::{TableBuilder, Column};
use egui_plot::{Plot, Line, PlotPoints, Legend};
use serde::Deserialize;
use serde_json::Value;
use web_sys::window;

/// Row object for table data
#[derive(Deserialize)]
pub struct Row {
    id: u32,
    timestamp: String,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    accel_x: f64,
    accel_y: f64,
    accel_z: f64,
    gyro_x: f64,
    gyro_y: f64,
    gyro_z: f64,
    dac_1: f64,
    dac_2: f64,
    dac_3: f64,
    dac_4: f64,
}

/// Blob object for mapping data_blob in response
#[derive(Deserialize)]
pub struct Blob {
    lat: f64,
    lon: f64,
    alt: f64,
    accel_x: f64,
    accel_y: f64,
    accel_z: f64,
    gyro_x: f64,
    gyro_y: f64,
    gyro_z: f64,
    dac_1: f64,
    dac_2: f64,
    dac_3: f64,
    dac_4: f64,
}

/// Individual response object 
#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Row2 {
    datetime: String,
    id: i64,
    data_blob: Value,
}

/// Response vector
#[derive(Deserialize)]
pub struct DataResponse {
    pub datapoints: Vec<Row2>,
}

/// Theme dropdown/toggle
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    LightMode,
    DarkMode
}

/// Data type dropdown
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Selection {
    SensorData,
    LocData,
    AccelData,
    GyroData,
    DacData,
}

/// Display type dropdown
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DisplayType {
    All,
    Table,
    Graph,
    Map
}

/// Main window for data display
pub struct DataWindow {
    table_headers: Vec<String>,
    table_data: Vec<Row>,
    datapoints: Vec<Row2>,

    dropdown: Selection,
    theme_dropdown: Theme,
    display_dropdown: DisplayType,
    fullscreen: bool,
    current_page: usize,
    direction: bool,
    direction_text: String,

    loaded: bool,
    formatted: bool,
    prev_session: String,
    current_session: *mut String,
    last_refresh: f64,
    last_row: usize,
    last_datetime: Option<String>,
    first_fetch: bool,
}

impl DataWindow {
    pub fn new(current_session: *mut String) -> Self {

        // Hardcoded headers
        let headers = vec![
            "id".to_string(),
            "timestamp".to_string(),
            "latitude".to_string(),
            "longitude".to_string(),
            "altitude".to_string(),
            "accel_x".to_string(),
            "accel_y".to_string(),
            "accel_z".to_string(),
            "gyro_x".to_string(),
            "gyro_y".to_string(),
            "gyro_z".to_string(),
            "dac_1".to_string(),
            "dac_2".to_string(),
            "dac_3".to_string(),
            "dac_4".to_string(),
        ];

        // Initialize
        DataWindow {
            table_headers: headers,
            table_data: Vec::new(),
            datapoints: Vec::new(),

            dropdown: Selection::AccelData,
            theme_dropdown: Theme::DarkMode,
            display_dropdown: DisplayType::Table,
            fullscreen: false,
            current_page: 0,
            direction: false,
            direction_text: "Sort: ascending v".to_string(),

            loaded: false,
            formatted: false,
            prev_session: String::new(),
            current_session,
            last_refresh: 0.0,
            last_row: 0,
            last_datetime: None,
            first_fetch: true,
        }
    }

    /// Function to issue request and handle response from tcp server
    pub fn load_data(&mut self) {
        let current_session_string = unsafe { (*self.current_session).clone() };

        // Clear flags and saved values for new session
        if current_session_string != self.prev_session {
            self.prev_session = current_session_string.clone();
            self.loaded = false;
            self.first_fetch = true;
            self.last_datetime = None; 
            self.last_row = 0;   
            self.datapoints.clear();  
            self.table_data.clear(); 
        }
    
        self.loaded = true; 

        let current_datetime_string = self.last_datetime.clone().unwrap_or("2025-01-01T00:00:00.000".to_string());
    
        // Used to get data out of async blocks 
        let formatted_ptr: *mut bool = &mut self.formatted;
        let data_ptr: *mut Vec<Row2> = &mut self.datapoints;
        let last_datetime_ptr: *mut Option<String> = &mut self.last_datetime;
    
        let client = client::get_client();
    
        // Only runs on the first fetch when loading a session
        if self.first_fetch {
            wasm_bindgen_futures::spawn_local(async move {
                let (status, val) =
                    session_sensor_data::view_datapoints_by_session_id(&client, &current_session_string).await;
    
                if status == 200 {
                    web_sys::console::log_1(&format!("First data loaded. Status: {}", status).into());
    
                    if let Some(val) = val {
                        match serde_json::from_value::<DataResponse>(val) {
                            Ok(parsed) => {
                                let datapoints = parsed.datapoints;
                                let last_datetiime = datapoints.last().map(|row| row.datetime.clone());
                                unsafe {
                                    *data_ptr = datapoints;
                                    *last_datetime_ptr = last_datetiime;
                                    *formatted_ptr = true;
                                }
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Failed to parse data response: {}", e).into(),);
                            }
                        }
                    }
                } else {
                    web_sys::console::log_1( &format!("Data fetch failed. Status: {}", status).into());
                }
            });
        } 

        // Refreshes to fetch new data
        else {
            wasm_bindgen_futures::spawn_local(async move {
                let (status, val) = session_sensor_data::view_all_datapoints_by_id_datetime(&client, &current_session_string, &current_datetime_string,).await;
            
                if status == 200 {
                    web_sys::console::log_1(&format!("Data loaded. Status: {}", status).into());
            
                    if let Some(val) = val {
                        match serde_json::from_value::<DataResponse>(val) {
                            Ok(parsed) => {
                                let new_datapoints = parsed.datapoints;
                                unsafe {
                                    (*data_ptr).extend(new_datapoints);
                                    *last_datetime_ptr = (*data_ptr).last().map(|row| row.datetime.clone());
                                    *formatted_ptr = true;
                                }
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Failed to parse data response: {}", e).into());
                            }
                        }
                    }
                } else {
                    web_sys::console::log_1(&format!("Data fetch failed. Status: {}", status).into());
                }
            });
        }
    
        self.first_fetch = false;
    }

    /// Function to format the JSON response and insert it into table data
    pub fn format_data(&mut self) {
        web_sys::console::log_1(&format!("Formatting").into());

        // Make sure not to break if datapoints get removed
        if self.last_row > self.datapoints.len() {
            self.last_row = 0;
            self.table_data.clear();
        }

        // Only gather new datapoints to format 
        let new_datapoints = &self.datapoints[self.last_row..];

        // Change to ascending for insertion
        if self.direction == false {
            self.table_data.reverse();
        }

        // Iterate over new datapoints as Blobs and push to table data
        for (i, row) in new_datapoints.iter().enumerate() {
            match serde_json::from_value::<Blob>(row.data_blob.clone()) {
                Ok(parsed) => {
                    self.table_data.push(Row {
                        id: (self.last_row + i) as u32,         // Used for readability since the actual id will always be the same
                        timestamp: row.datetime.clone(),
                        latitude: parsed.lat,
                        longitude: parsed.lon,
                        altitude: parsed.alt,
                        accel_x: parsed.accel_x,
                        accel_y: parsed.accel_y,
                        accel_z: parsed.accel_z,
                        gyro_x: parsed.gyro_x,
                        gyro_y: parsed.gyro_y,
                        gyro_z: parsed.gyro_z,
                        dac_1: parsed.dac_1,
                        dac_2: parsed.dac_2,
                        dac_3: parsed.dac_3,
                        dac_4: parsed.dac_4,
                    });
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Failed to format data: {}", e).into());
                }
            };
        }

        // Save last row for performance
        self.last_row = self.datapoints.len();

        // Change back 
        if self.direction == false {
            self.table_data.reverse();
        }
    }

    /// Draw the data window
    pub fn draw(&mut self, ctx: &eframe::egui::Context, ) -> () {

        // Get current time from window
        let current_time = match window() {
            Some(w) => match w.performance() {
                Some(p) => p.now(),
                None => 0.0,
            },
            None => 0.0,
        };

        // Load data every 1 second OR every time current session changes
        if current_time - self.last_refresh >= 1000.0 || !self.loaded {
            self.load_data();
            self.last_refresh = current_time;
        }

        // Format data if unformatted
        if self.formatted {
            self.format_data();
            self.formatted = false;
        }

        ctx.request_repaint();

        eframe::egui::Window::new("Data Window")
        .resizable(true)
        .auto_sized()
        .movable(!self.fullscreen)
        .show(ctx, |ui| {

            unsafe {
                ui.label( format!("Current session: {}", *self.current_session));
            }

            // Set fullscreen size
            if self.fullscreen {
                ui.set_min_size(ctx.screen_rect().size());
                ctx.request_repaint();
            }
            
            // Set visuals based on theme
            match self.theme_dropdown {
                Theme::LightMode => {
                    ctx.set_visuals(egui::Visuals::light());
                }
                Theme::DarkMode => {
                    ctx.set_visuals(egui::Visuals::dark());
                }
            }
        
            // Top bar setup
            Frame::none()
                .fill(egui::Color32::LIGHT_GRAY)
                .inner_margin(egui::Margin::symmetric(10.0, 10.0))
                .show(ui, |ui| {
                    if self.fullscreen == true {
                        ui.set_width(ui.available_width());
                    }
                    else {
                        ui.set_width(940.0);
                    }
                    
                    // Dropdown menus
                    ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label("Theme:");
                        ComboBox::from_id_salt("Theme")
                            .selected_text(match self.theme_dropdown {
                                Theme::LightMode => "Light Mode",
                                Theme::DarkMode => "Dark Mode",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.theme_dropdown, Theme::LightMode, "Light Mode");
                                ui.selectable_value(&mut self.theme_dropdown, Theme::DarkMode, "Dark Mode");
                            });
                        ui.add_space(20.0);
                        ui.label("Sensor:");
                        ComboBox::from_id_salt("SensorData")
                            .selected_text(match self.dropdown {
                                Selection::SensorData => "All Data",
                                Selection::LocData => "Location Data",
                                Selection::AccelData => "Acceleration Data",
                                Selection::GyroData => "Gyroscopic Data",
                                Selection::DacData => "Dac Data",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.dropdown, Selection::SensorData, "All Data");
                                ui.selectable_value(&mut self.dropdown, Selection::LocData, "Location Data");
                                ui.selectable_value(&mut self.dropdown, Selection::AccelData, "Acceleration Data");
                                ui.selectable_value(&mut self.dropdown, Selection::GyroData, "Gyroscopic Data");
                                ui.selectable_value(&mut self.dropdown, Selection::DacData, "Dac Data");
                            });
                        ui.add_space(20.0);
                        ui.label("Display Type:");
                        ComboBox::from_id_salt("DisplayType")
                            .selected_text(match self.display_dropdown {
                                DisplayType::All => "All",
                                DisplayType::Table => "Table",
                                DisplayType::Graph => "Graph",
                                DisplayType::Map => "Map",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.display_dropdown, DisplayType::All, "All");
                                ui.selectable_value(&mut self.display_dropdown, DisplayType::Table, "Table");
                                ui.selectable_value(&mut self.display_dropdown, DisplayType::Graph, "Graph");
                                ui.selectable_value(&mut self.display_dropdown, DisplayType::Map, "Map");
                            });

                        // Toggle fullscreen mode
                        if ui.button("Fullscreen").clicked() {
                            self.fullscreen = !self.fullscreen;
                        }

                        // Page controls
                        let last_page = (self.table_data.len() + 9) / 10;

                        if ui.button("<").clicked() && self.current_page > 0 {
                            self.current_page -= 1;
                        }

                        ui.label(format!("Page {}/{}", self.current_page + 1, last_page));

                        if ui.button(">").clicked() && self.current_page + 1 < last_page {
                            self.current_page += 1;
                        }

                        // Sort direction
                        if ui.button(&self.direction_text).clicked() {
                            self.direction = !self.direction;
                            
                            if self.direction_text == "Sort: descending v" {
                                self.direction_text = "Sort: ascending ^".to_string();
                            }
                            else {
                                self.direction_text = "Sort: descending v".to_string();
                            }
                            
                            self.table_data.reverse();
                        }
                    });
                });
        
            // Main window setup
            let mut show_table = true;
            let mut show_graph = true;
            let mut show_map = true;
    
            Frame::none()
                .outer_margin(egui::Margin::symmetric(10.0, 10.0))
                .show(ui, |ui| {
                    if self.fullscreen == true {
                        ui.set_width(ui.available_width());
                    }
                    else {
                        ui.set_width(940.0);
                    }
                    
                    // Display type selection. Different data types will have different options available
                    match self.display_dropdown {
                        DisplayType::All => {
                            show_table = true;
                            show_graph = true;
                            show_map = true;
                        }
                        DisplayType::Table => {
                            show_table = true;
                            show_graph = false;
                            show_map = false;
                        }
                        DisplayType::Graph => {
                            show_table = false;
                            show_graph = true;
                            show_map = false;
                        }
                        DisplayType::Map => {
                            show_table = false;
                            show_graph = false;
                            show_map = true;
                        }
                    }
    
                    // Table drawing for each data type
                    match self.dropdown {
                        Selection::SensorData => {
                            if show_table == true {
                                ui.heading("Sensor Data:");
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .columns(Column::auto(), self.table_headers.len())
                                    .header(30.0, |mut header| {
                                        for h in &self.table_headers {
                                            header.col(|ui| {
                                                ui.heading(h);
                                            });
                                        }
                                    })
                                    .body(|mut body| {
                                        let start_row = self.current_page * 10;
                                        let end_row = (start_row + 10).min(self.table_data.len());
                                        for r in &self.table_data[start_row..end_row] {
                                            body.row(20.0, |mut row_ui| {
                                                row_ui.col(|ui| { ui.label(r.id.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.24}",r.timestamp.clone())); });
                                                row_ui.col(|ui| { ui.label(r.latitude.to_string()); });
                                                row_ui.col(|ui| { ui.label(r.longitude.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.altitude)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_x)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.accel_y)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.accel_z)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.gyro_x)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.gyro_y)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.gyro_z)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.dac_1)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.dac_2)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.dac_3)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.dac_4)); });
                                            });
                                        }  
                                    });
                                ui.separator();  
                            }
                        }
                        Selection::LocData => {
                            if show_table == true {
                                ui.heading("Sensor Data:");
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .columns(Column::auto(), 5)
                                    .header(30.0, |mut header| {
                                        for h in self.table_headers.iter().take(5) {
                                            header.col(|ui| {
                                                ui.heading(h);
                                            });
                                        }
                                    })
                                    .body(|mut body| {
                                        let start_row = self.current_page * 10;
                                        let end_row = (start_row + 10).min(self.table_data.len());
                                        for r in &self.table_data[start_row..end_row] {
                                            body.row(20.0, |mut row_ui| {
                                                row_ui.col(|ui| { ui.label(r.id.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.24}",r.timestamp.clone())); });
                                                row_ui.col(|ui| { ui.label(r.latitude.to_string()); });
                                                row_ui.col(|ui| { ui.label(r.longitude.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.altitude)); });
                                            });
                                        }  
                                    });
                                ui.separator();  
                            }
                        }
                        Selection::AccelData => {
                            if show_table == true {
                                ui.heading("Sensor Data:");
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .columns(Column::auto(), 5)
                                    .header(30.0, |mut header| {
                                        for h in self.table_headers.iter().take(2) {
                                            header.col(|ui| {
                                                ui.heading(h);
                                            });
                                        }
                                        for h in self.table_headers.iter().skip(5).take(3) {
                                            header.col(|ui| {
                                                ui.heading(h);
                                            });
                                        }
                                    })
                                    .body(|mut body| {
                                        let start_row = self.current_page * 10;
                                        let end_row = (start_row + 10).min(self.table_data.len());
                                        for r in &self.table_data[start_row..end_row] {
                                            body.row(20.0, |mut row_ui| {
                                                row_ui.col(|ui| { ui.label(r.id.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.24}", r.timestamp.clone())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_x)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_y)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_z)); });
                                            });
                                        }  
                                    });

                                ui.add_space(10.0);
                                let len = self.table_data.len() as f64;
                                let avg_accel_x: f64 = self.table_data.iter().map(|row| row.accel_x).sum::<f64>();
                                let avg_accel_y: f64 = self.table_data.iter().map(|row| row.accel_y).sum::<f64>();
                                let avg_accel_z: f64 = self.table_data.iter().map(|row| row.accel_z).sum::<f64>();
                                ui.label(format!("Average Acceleration in X: {:.4}", avg_accel_x/len));
                                ui.label(format!("Average Acceleration in Y: {:.4}", avg_accel_y/len));
                                ui.label(format!("Average Acceleration in Z: {:.4}", avg_accel_z/len));
                                ui.separator();  
                            }

                            // Graph drawing
                            if show_graph == true {
                                ui.add_space(10.0);
                                ui.heading("Sensor Graph:");

                                let accel_x: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.accel_x]).collect();

                                let accel_y: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.accel_y]).collect();

                                let accel_z: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.accel_z]).collect();

                                Plot::new("accel_graph")
                                    .legend(Legend::default())
                                    .x_axis_label("ID")
                                    .y_axis_label("Acceleration")
                                    .width(800.0)
                                    .height(300.0)
                                    .show(ui, |ui| {
                                        ui.line(Line::new(accel_x).name("Accel X").color(egui::Color32::RED));
                                        ui.line(Line::new(accel_y).name("Accel Y").color(egui::Color32::GREEN));
                                        ui.line(Line::new(accel_z).name("Accel Z").color(egui::Color32::BLUE));
                                    });
                            }    
                        }
                        Selection::GyroData => {
                            if show_table == true {
                                ui.heading("Sensor Data:");
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .columns(Column::auto(), 5)
                                    .header(30.0, |mut header| {
                                        for h in self.table_headers.iter().take(2) {
                                            header.col(|ui| {
                                                ui.heading(h);
                                            });
                                        }
                                        for h in self.table_headers.iter().skip(8).take(3) {
                                            header.col(|ui| {
                                                ui.heading(h);
                                            });
                                        }
                                    })
                                    .body(|mut body| {
                                        let start_row = self.current_page * 10;
                                        let end_row = (start_row + 10).min(self.table_data.len());
                                        for r in &self.table_data[start_row..end_row] {
                                            body.row(20.0, |mut row_ui| {
                                                row_ui.col(|ui| { ui.label(r.id.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.24}", r.timestamp.clone())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.gyro_x)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.gyro_y)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.gyro_z)); });
                                            });
                                        }  
                                    });

                                ui.add_space(10.0);
                                let len = self.table_data.len() as f64;
                                let avg_gyro_x: f64 = self.table_data.iter().map(|row| row.gyro_x).sum::<f64>();
                                let avg_gyro_y: f64 = self.table_data.iter().map(|row| row.gyro_y).sum::<f64>();
                                let avg_gyro_z: f64 = self.table_data.iter().map(|row| row.gyro_z).sum::<f64>();
                                ui.label(format!("Average Gyro in X: {:.4}", avg_gyro_x/len));
                                ui.label(format!("Average Gyro in Y: {:.4}", avg_gyro_y/len));
                                ui.label(format!("Average Gyro in Z: {:.4}", avg_gyro_z/len));
                                ui.separator();  
                            }

                            // Graph drawing
                            if show_graph == true {
                                ui.add_space(10.0);
                                ui.heading("Sensor Graph:");

                                let gyro_x: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.gyro_x]).collect();

                                let gyro_y: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.gyro_y]).collect();

                                let gyro_z: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.gyro_z]).collect();

                                Plot::new("gyro_graph")
                                    .legend(Legend::default())
                                    .x_axis_label("ID")
                                    .y_axis_label("Gyro")
                                    .width(800.0)
                                    .height(300.0)
                                    .show(ui, |ui| {
                                        ui.line(Line::new(gyro_x).name("Gyro X").color(egui::Color32::RED));
                                        ui.line(Line::new(gyro_y).name("Gyro Y").color(egui::Color32::GREEN));
                                        ui.line(Line::new(gyro_z).name("Gyro Z").color(egui::Color32::BLUE));
                                    });
                            }    
                        }
                        Selection::DacData => {
                            if show_table == true {
                                ui.heading("Sensor Data:");
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .columns(Column::auto(), 6)
                                    .header(30.0, |mut header| {
                                        for h in self.table_headers.iter().take(2) {
                                            header.col(|ui| {
                                                ui.heading(h);
                                            });
                                        }
                                        for h in self.table_headers.iter().skip(11).take(4) {
                                            header.col(|ui| {
                                                ui.heading(h);
                                            });
                                        }
                                    })
                                    .body(|mut body| {
                                        let start_row = self.current_page * 10;
                                        let end_row = (start_row + 10).min(self.table_data.len());
                                        for r in &self.table_data[start_row..end_row] {
                                            body.row(20.0, |mut row_ui| {
                                                row_ui.col(|ui| { ui.label(r.id.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.24}", r.timestamp.clone())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.dac_1)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.dac_2)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.dac_3)); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.dac_4)); });
                                            });
                                        }  
                                    });

                                ui.add_space(10.0);
                                let len = self.table_data.len() as f64;
                                let avg_dac_1: f64 = self.table_data.iter().map(|row| row.dac_1).sum::<f64>();
                                let avg_dac_2: f64 = self.table_data.iter().map(|row| row.dac_2).sum::<f64>();
                                let avg_dac_3: f64 = self.table_data.iter().map(|row| row.dac_3).sum::<f64>();
                                let avg_dac_4: f64 = self.table_data.iter().map(|row| row.dac_4).sum::<f64>();
                                ui.label(format!("Average Dac 1: {:.4}", avg_dac_1/len));
                                ui.label(format!("Average Dac 2: {:.4}", avg_dac_2/len));
                                ui.label(format!("Average Dac 3: {:.4}", avg_dac_3/len));
                                ui.label(format!("Average Dac 4: {:.4}", avg_dac_4/len));
                                ui.separator();  
                            }

                            // Graph drawing
                            if show_graph == true {
                                ui.add_space(10.0);
                                ui.heading("Sensor Graph:");

                                let dac_1: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.dac_1]).collect();

                                let dac_2: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.dac_2]).collect();

                                let dac_3: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.dac_3]).collect();

                                let dac_4: PlotPoints = self.table_data.iter().enumerate()
                                    .map(|(i, row)| [(i + 1) as f64, row.dac_4]).collect();

                                Plot::new("dac_graph")
                                    .legend(Legend::default())
                                    .x_axis_label("ID")
                                    .y_axis_label("Dac")
                                    .width(800.0)
                                    .height(300.0)
                                    .show(ui, |ui| {
                                        ui.line(Line::new(dac_1).name("Dac 1").color(egui::Color32::RED));
                                        ui.line(Line::new(dac_2).name("Dac 2").color(egui::Color32::GREEN));
                                        ui.line(Line::new(dac_3).name("Dac 3").color(egui::Color32::BLUE));
                                        ui.line(Line::new(dac_4).name("Dac 4").color(egui::Color32::YELLOW));
                                    });
                            }    
                        }
                    }
                });        
        });
    }
}
