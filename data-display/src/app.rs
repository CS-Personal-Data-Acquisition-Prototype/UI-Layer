use eframe::egui::CentralPanel;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DisplayApp {
    /* This how you opt-out of serialization of a field
    #[serde(skip)]
    value: f32*/
}

impl Default for DisplayApp {
    fn default() -> Self {
        Self {}
    }
}

impl DisplayApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        //TODO: Load previous app state if any (must enable the `persistence` feature)

        Default::default()
    }
}

impl eframe::App for DisplayApp {
    //TODO: fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, World!");
        });
    }
}
