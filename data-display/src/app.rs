use eframe::{
    egui::{CentralPanel, Color32, Pos2, Rect, Shape, Ui},
    emath,
};
use std::f32::consts::PI;

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

    fn rgb_hsv(color: Color32) -> (f32, f32, f32) {
        let r = color.r() as f32 / 255.;
        let g = color.g() as f32 / 255.;
        let b = color.b() as f32 / 255.;

        let c_max: f32 = f32::max(r, f32::max(g, b));
        let c_min = f32::min(r, f32::min(g, b));
        let delta = c_max - c_min;

        let hue: f32 = if c_max == r {
            60. * (((g - b) / delta) % 6.)
        } else if c_max == g {
            60. * (((b - r) / delta) + 2.)
        } else if c_max == b {
            60. * (((r - g) / delta) + 4.)
        } else {
            0.
        };
        let saturation: f32 = match c_max {
            0. => 0.,
            _ => delta / c_max,
        };

        (hue, saturation * 100., c_max * 100.)
    }

    fn hsv_rgb(color: (f32, f32, f32)) -> Color32 {
        let c = color.2 * color.1;
        let x = c * (1. - f32::abs(((color.1 / 60.) % 2.) - 1.));
        let m = color.2 - c;

        let (r, g, b) = match color.1 {
            0.0..=60.0 => (c, x, 0.),
            60.0..=120.0 => (x, c, 0.),
            120.0..=180.0 => (0., c, x),
            180.0..=240.0 => (0., x, c),
            240.0..=300.0 => (x, 0., c),
            _ => (c, 0., x),
        };
        Color32::from_rgb(
            ((r + m) * 255.) as u8,
            ((g + m) * 255.) as u8,
            ((b + m) * 255.) as u8,
        )
    }

    fn spiral(&mut self, ui: &mut Ui) {
        ui.ctx().request_repaint();
        let time = ui.input(|i| i.time);
        let speed = 0.05;

        let radius: f32 = 2.;
        let p = 1;
        let q = 10;
        let phase: f32 = p as f32 / q as f32;
        let range: f32 = 2. * PI * q as f32 / p as f32;
        let pos = |x: f32, operations: Option<(emath::Vec2, emath::Vec2)>| -> Pos2 {
            let (translate, scale) =
                operations.unwrap_or((emath::Vec2 { x: 0., y: 0. }, emath::Vec2 { x: 1., y: 1. }));
            Pos2::new(
                scale.x * (radius * f32::cos(x) + f32::cos(x * phase) + translate.x),
                scale.y * (radius * f32::sin(x) - f32::sin(x * phase) + translate.y),
            )
        };

        //draw shapes (circles) at points in range [0, range]
        let rect = ui.painter().clip_rect();
        let max = pos(range, None).x;
        let dims = rect.size();
        let ratio = rect.square_proportions() / rect.square_proportions().max_elem();
        let translate = emath::Vec2 { x: max, y: max };
        let scale = dims / (2. * max);
        //log::info!("max: {max}");
        //log::info!("dims: {dims}");

        let n = 500;
        let spacing = range / n as f32;
        let points = (0..n)
            .filter_map(|i| {
                let point = pos(
                    (i as f32 * spacing) + (time * speed) as f32,
                    Some((translate * rect.square_proportions(), scale * ratio.yx())),
                );
                if rect.intersects(Rect::from_pos(point)) {
                    Some(Shape::circle_filled(
                        point,
                        5.,
                        DisplayApp::hsv_rgb(DisplayApp::rgb_hsv(Color32::GREEN)), //TODO: change color based on rotation as hue
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<Shape>>();

        ui.painter().extend(points);
    }
}

impl eframe::App for DisplayApp {
    //TODO: fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.spiral(ui);
        });
    }
}
