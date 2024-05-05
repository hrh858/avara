use egui::{pos2, Color32};

pub struct Renderer<'a> {
    ui: &'a mut egui::Ui,
    painter: egui::Painter,
    surface: RendererSurface,
}

struct RendererSurface {
    width: f32,
    height: f32,
    ratio: f32,
}

impl<'a> Renderer<'a> {
    pub fn new(ui: &'a mut egui::Ui) -> Self {
        let available_rect = ui.available_rect_before_wrap();
        let width = available_rect.width().round();
        let height = available_rect.height().round();
        let ratio = width / height;

        Self {
            surface: RendererSurface {
                width,
                height,
                ratio,
            },
            painter: egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            ),
            ui,
        }
    }
    pub fn update(&mut self) {
        self.draw_bg();
        self.draw_grid();
        self.draw_rect(40, 0, 40, 60, Color32::RED);
        self.draw_rect(300, 80, 100, 300, Color32::BLUE);

        self.ui.expand_to_include_rect(self.painter.clip_rect());
    }

    fn draw_bg(&self) {
        self.painter.rect_filled(
            egui::Rect {
                min: pos2(0.0, 0.0),
                max: pos2(self.surface.width, self.surface.height),
            },
            0.0,
            Color32::from_rgb(26, 26, 26),
        );
    }

    fn draw_grid(&self) {
        for x in 0..(self.surface.width as u16) {
            for y in 0..(self.surface.height as u16) {
                if x % 30 == 0 || y % 30 == 0 {
                    self.draw_pixel(x, y);
                }
            }
        }
    }

    fn draw_pixel(&self, x: u16, y: u16) {
        self.painter.rect_filled(
            egui::Rect {
                min: pos2(x as f32, y as f32),
                max: pos2((x + 1) as f32, (y + 1) as f32),
            },
            0.0,
            Color32::WHITE,
        );
    }

    fn draw_rect(&self, x: u16, y: u16, width: u16, height: u16, color: Color32) {
        self.painter.rect_filled(
            egui::Rect {
                min: pos2(x as f32, y as f32),
                max: pos2((x + width) as f32, (y + height) as f32),
            },
            0.0,
            color,
        );
    }
}
