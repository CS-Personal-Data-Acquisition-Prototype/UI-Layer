//! Main data display window
//!

extern crate client;
use client::api::{session_sensor_data};

use eframe::egui::{ComboBox, Frame};
use egui_extras::{TableBuilder, Column};
use egui_plot::{Plot, Line, PlotPoints, Legend};
use serde::Deserialize;

/// Row object for csv data (likely to change when connected to backend)
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

#[derive(Deserialize)]
pub struct Row2 {
    datetime: String,
    id: i64,
    data_blob: String,
}

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
    AccelData
}

/// Display type dropdown
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DisplayType {
    All,
    Table,
    Graph,
    Map
}

/// Main window for data display. Has top bar with dropdown selectors, table view, and graphiical view.
pub struct DataWindow {
    table_headers: Vec<String>,
    table_data: Vec<Row>,
    datapoints: Vec<Row2>,
    dropdown: Selection,
    theme_dropdown: Theme,
    display_dropdown: DisplayType,
    fullscreen: bool,
    loaded: bool,
    prev_session: String,
    current_session: *mut String,
}

impl DataWindow {
    pub fn new(current_session: *mut String) -> Self {
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
            loaded: false,
            prev_session: String::new(),
            current_session,
        }
    }

    pub fn load_data(&mut self) {
        self.loaded = true;

        let current_session_string = unsafe { (*self.current_session).clone() };

        let data_ptr: *mut Vec<Row2> = &mut self.datapoints;

        let client = client::get_client();

        wasm_bindgen_futures::spawn_local(async move {
            let (status, val) = session_sensor_data::view_datapoints_by_session_id(&client, &current_session_string).await;
            
            if status == 200 {
                web_sys::console::log_1( &format!("Data loaded. Status: {}", status).into() );

                if let Some(val) = val {
                    match serde_json::from_value::<DataResponse>(val) {
                        Ok(parsed) => {
                            unsafe {
                                *data_ptr = parsed.datapoints;
                            }
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("Failed to parse Data: {}", e).into());
                        }
                    }
                }
                
            } else {
                web_sys::console::log_1( &format!("Data failed. Status: {}", status).into());
            }
        });

        
    }

    pub fn format_data(&mut self) {
        self.table_data.clear();

        for row in &self.datapoints {
            let col: Vec<&str> = row.data_blob.split(',').collect();

            let blob = Blob {
                lat: col[0].trim().parse::<f64>().unwrap_or(0.0),
                lon: col[1].trim().parse::<f64>().unwrap_or(0.0),
                alt: col[2].trim().parse::<f64>().unwrap_or(0.0),
                accel_x: col[3].trim().parse::<f64>().unwrap_or(0.0),
                accel_y: col[4].trim().parse::<f64>().unwrap_or(0.0),
                accel_z: col[5].trim().parse::<f64>().unwrap_or(0.0),
                gyro_x: col[6].trim().parse::<f64>().unwrap_or(0.0),
                gyro_y: col[7].trim().parse::<f64>().unwrap_or(0.0),
                gyro_z: col[8].trim().parse::<f64>().unwrap_or(0.0),
                dac_1: col[9].trim().parse::<f64>().unwrap_or(0.0),
                dac_2: col[10].trim().parse::<f64>().unwrap_or(0.0),
                dac_3: col[11].trim().parse::<f64>().unwrap_or(0.0),
                dac_4: col[12].trim().parse::<f64>().unwrap_or(0.0),
            };

            self.table_data.push(Row {
                id: row.id as u32,
                timestamp: row.datetime.clone(),
                latitude: blob.lat,
                longitude: blob.lon,
                altitude: blob.alt,
                accel_x: blob.accel_x,
                accel_y: blob.accel_y,
                accel_z: blob.accel_z,
                gyro_x: blob.gyro_x,
                gyro_y: blob.gyro_y,
                gyro_z: blob.gyro_z,
                dac_1: blob.dac_1,
                dac_2: blob.dac_2,
                dac_3: blob.dac_3,
                dac_4: blob.dac_4,
            });
        }
    }

    /// Draw the data window
    pub fn draw(&mut self, ctx: &eframe::egui::Context, ) -> () {
        let current_session_string = unsafe { (*self.current_session).clone() };

        if current_session_string != self.prev_session {
            self.prev_session = current_session_string.clone();
            self.loaded = false
        }

        if !self.loaded { 
            self.load_data(); 
        }

        self.format_data(); 
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
        
            // Dropdown pannel setup
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
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.dropdown, Selection::SensorData, "All Data");
                                ui.selectable_value(&mut self.dropdown, Selection::LocData, "Location Data");
                                ui.selectable_value(&mut self.dropdown, Selection::AccelData, "Acceleration Data");
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
                                        for r in &self.table_data {
                                            body.row(20.0, |mut row_ui| {
                                                row_ui.col(|ui| { ui.label(r.id.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.24}",r.timestamp.clone())); });
                                                row_ui.col(|ui| { ui.label(r.latitude.to_string()); });
                                                row_ui.col(|ui| { ui.label(r.longitude.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.altitude.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_x.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.accel_y.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.accel_z.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.gyro_x.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.gyro_y.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.gyro_z.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.dac_1.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.dac_2.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.dac_3.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.dac_4.to_string())); });
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
                                        for r in &self.table_data {
                                            body.row(20.0, |mut row_ui| {
                                                row_ui.col(|ui| { ui.label(r.id.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.24}",r.timestamp.clone())); });
                                                row_ui.col(|ui| { ui.label(r.latitude.to_string()); });
                                                row_ui.col(|ui| { ui.label(r.longitude.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}",r.altitude.to_string())); });
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
                                        for r in &self.table_data {
                                            body.row(20.0, |mut row_ui| {
                                                row_ui.col(|ui| { ui.label(r.id.to_string()); });
                                                row_ui.col(|ui| { ui.label(format!("{:.24}", r.timestamp.clone())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_x.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_y.to_string())); });
                                                row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_z.to_string())); });
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
                    }
                });        
        });
    }
}
