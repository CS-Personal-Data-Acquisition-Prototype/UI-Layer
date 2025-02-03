use eframe::egui::{CentralPanel, Grid};
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

pub struct DisplayApp {
    table_headers: Vec<String>,
    table_data: Vec<Row>,
}

impl DisplayApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

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
        }
    }
}

impl eframe::App for DisplayApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Sensor Data:");

            Grid::new("table_grid")
                .num_columns(14) 
                .show(ui, |ui| {
                    for header in &self.table_headers {
                            ui.label(header);
                        }
                        ui.end_row();

                    for row in &self.table_data {
                        ui.label(&row.id.to_string());
                        ui.label(&row.timestamp);
                        ui.label(&row.latitude.to_string());
                        ui.label(&row.longitude.to_string());
                        ui.label(&row.altitude.to_string());
                        ui.label(&row.accel_x.to_string());
                        ui.label(&row.accel_y.to_string());
                        ui.label(&row.accel_z.to_string());
                        ui.label(&row.gyro_x.to_string());
                        ui.label(&row.gyro_y.to_string());
                        ui.label(&row.gyro_z.to_string());
                        ui.label(&row.dac_1.to_string());
                        ui.label(&row.dac_2.to_string());
                        ui.label(&row.dac_3.to_string());
                        ui.label(&row.dac_4.to_string());
                        ui.end_row();
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
