mod data;

pub struct DisplayApp {
    window_data: data::DataWindow,
}

impl Default for DisplayApp {
    fn default() -> Self {
        Self {
            window_data: data::DataWindow::new(),
        }
    }
}

impl DisplayApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for DisplayApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.window_data.draw(ctx);
    }
}
