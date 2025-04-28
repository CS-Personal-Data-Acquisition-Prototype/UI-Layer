use eframe::egui::{CentralPanel, ComboBox, TopBottomPanel, Frame};
use egui_extras::{TableBuilder, Column};
use egui_plot::{Plot, Line, PlotPoints, Legend};
use serde::Deserialize;
use std::io::Cursor;
use csv::ReaderBuilder;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    LightMode,
    DarkMode
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Selection {
    SensorData,
    LocData,
    AccelData
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DisplayType {
    All,
    Table,
    Graph,
    Map
}

pub struct DisplayApp {
    table_headers: Vec<String>,
    table_data: Vec<Row>,
    dropdown: Selection,
    theme_dropdown: Theme,
    display_dropdown: DisplayType
}

impl DisplayApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {

        let csv_data = include_str!("../temp_data/mockdata.csv");
        
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(Cursor::new(csv_data));

        let header_row = rdr.headers().expect("Error reading headers").clone();
        
        let mut data: Vec<Row> = Vec::new();
        let mut headers: Vec<String> = Vec::new();
        
        for result in rdr.deserialize() {
            match result {
                Ok(row) => data.push(row),
                Err(e) => eprintln!("Error reading CSV: {e}"),
            }
        }

        for h in header_row.iter() {
            headers.push(h.to_string());
        }
        
        DisplayApp {
            table_headers: headers,
            table_data: data,
            dropdown: Selection::SensorData,
            theme_dropdown: Theme::DarkMode,
            display_dropdown: DisplayType::All
        }
    }
}

impl eframe::App for DisplayApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        match self.theme_dropdown {
            Theme::LightMode => {
                ctx.set_visuals(egui::Visuals::light());
            }
            Theme::DarkMode => {
                ctx.set_visuals(egui::Visuals::dark());
            }
        }

        TopBottomPanel::top("top").show(ctx, |ui| {
            Frame::none()
                .fill(egui::Color32::LIGHT_GRAY)
                .inner_margin(egui::Margin::symmetric(10.0, 10.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
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
                    });
                })
        });

        CentralPanel::default().show(ctx, |ui| {
            let mut show_table = true;
            let mut show_graph = true;
            let mut show_map = true;

            Frame::none()
                .outer_margin(egui::Margin::symmetric(10.0, 10.0))
                .show(ui, |ui| {
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

                    match self.dropdown {
                        Selection::SensorData => {
                            if show_table == true {
                                //ui.add_space(10.0);
                                ui.heading("Sensor Data:");
                                egui::ScrollArea::vertical().id_salt("All")
                                .show(ui, |ui| {
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
                                });
                            }
                        }
                        Selection::LocData => {
                            if show_table == true {
                                //ui.add_space(10.0);
                                ui.heading("Sensor Data:");
                                egui::ScrollArea::vertical().id_salt("Loc")
                                .show(ui, |ui| {
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
                                });
                            }
                        }
                        Selection::AccelData => {
                            if show_table == true {
                                //ui.add_space(10.0);
                                ui.heading("Sensor Data:");
                                egui::ScrollArea::vertical().id_salt("Accel")
                                .show(ui, |ui| {
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
                                });
                            }
                            if show_graph == true {
                                egui::ScrollArea::vertical().id_salt("Graph")
                                .show(ui, |ui| {
                                    ui.add_space(10.0);
                                    ui.heading("Sensor Graph:");
    
                                    let accel_x: PlotPoints = self.table_data.iter()
                                        .map(|row| [row.id as f64, row.accel_x]).collect();
    
                                    let accel_y: PlotPoints = self.table_data.iter()
                                        .map(|row| [row.id as f64, row.accel_y]).collect();
    
                                    let accel_z: PlotPoints = self.table_data.iter()
                                        .map(|row| [row.id as f64, row.accel_z]).collect();
    
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
                                
                                });
                            }    
                        }
                    }
                })        
        });
    }
}
