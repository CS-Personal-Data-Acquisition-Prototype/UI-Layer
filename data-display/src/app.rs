use eframe::egui::{CentralPanel, ComboBox};
use egui_extras::{TableBuilder, Column};
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
pub enum Selection {
    SensorData,
    LocData,
    AccelData
}

pub struct DisplayApp {
    table_headers: Vec<String>,
    table_data: Vec<Row>,
    dropdown: Selection,
}

impl DisplayApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {

        let csv_data = include_str!("../data/mockdata.csv");
        
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
            dropdown: Selection::SensorData
        }
    }
}

impl eframe::App for DisplayApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sensor Data:");

            ComboBox::from_label("")
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
            
                match self.dropdown {
                    Selection::SensorData => {
                        egui::ScrollArea::vertical().show(ui, |ui| {
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
                        });
                    }
                    Selection::LocData => {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            TableBuilder::new(ui)
                                //.striped(true)
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
                            });
                    }
                    Selection::AccelData => {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            TableBuilder::new(ui)
                                //.striped(true)
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
                                            row_ui.col(|ui| { ui.label(format!("{:.24}",r.timestamp.clone())); });
                                            row_ui.col(|ui| { ui.label(format!("{:.6}", r.accel_x.to_string())); });
                                            row_ui.col(|ui| { ui.label(format!("{:.6}",r.accel_y.to_string())); });
                                            row_ui.col(|ui| { ui.label(format!("{:.6}",r.accel_z.to_string())); });
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
                            });
                    }
                }        
        });
    }
}
