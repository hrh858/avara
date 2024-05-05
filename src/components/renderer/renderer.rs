use egui::Color32;

use crate::GomuOriApp;

use super::{
    vector2::{vec2, Vector2},
    vector3::Vector3,
};

pub struct Renderer<'a> {
    ui: &'a mut egui::Ui,
    buffer: egui::ColorImage,
    surface: RendererSurface,
}

struct RendererSurface {
    width: f32,
    height: f32,
    ratio: f32,
}

impl<'a> Renderer<'a> {
    pub fn create(ui: &'a mut egui::Ui) -> Self {
        let available_rect = ui.available_rect_before_wrap();
        let width = available_rect.width().round();
        let height = available_rect.height().round();
        let ratio = width / height;

        let buffer = egui::ColorImage::new(
            // Make the buffer as big as the available space
            [width as usize, height as usize],
            // This will be the BG color
            Color32::from_rgb(26, 26, 26),
        );

        Self {
            buffer,
            surface: RendererSurface {
                width,
                height,
                ratio,
            },
            ui,
        }
    }

    pub fn draw(mut self, state: &GomuOriApp) -> egui::Response {
        // self.draw_grid(state.scale, Color32::WHITE);
        // self.draw_rect(0, 0, 500, 300, Color32::BLUE);
        // self.draw_rect(40, 50, 10, 10, Color32::RED);

        let width = self.surface.width;
        let height = self.surface.height;
        let scale = state.scale;

        if let Some(points) = &state.shape {
            points
                .into_iter()
                .map(|p| Self::project_point_orthographic(*p))
                .map(|p| Self::translate_point(p, width, height, scale as f32))
                // .for_each(|p| self.draw_rect(p.x, p.y, 3.0, 3.0, Color32::YELLOW))
            .for_each(|p| self.draw_pixel(p.x, p.y, Color32::YELLOW))
        }

        let options = egui::TextureOptions::LINEAR;
        let texture_handler = self
            .ui
            .ctx()
            .load_texture("renderer_buffer", self.buffer, options);
        self.ui.image(&texture_handler)
        // self.ui.expand_to_include_rect(self..clip_rect());
    }

    fn draw_grid(&mut self, scale: usize, color: Color32) {
        for x in 0..(self.surface.width as usize) {
            for y in 0..(self.surface.height as usize) {
                if x % (10 * scale) == 0 && y % (10 * scale) == 0 {
                    self.draw_pixel(x as f32, y as f32, color);
                }
            }
        }
    }

    fn draw_pixel(&mut self, x: f32, y: f32, color: Color32) {
        if !(x > (self.surface.width - 1.0) || y > (self.surface.height - 1.0)) {
            self.buffer.pixels[y as usize * self.surface.width as usize + x as usize] = color;
        }
    }

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color32) {
        for _x in (x as usize)..(x + width) as usize {
            for _y in (y as usize)..(y + height) as usize {
                self.draw_pixel(_x as f32, _y as f32, color)
            }
        }
    }

    fn project_point_orthographic(point: Vector3) -> Vector2 {
        vec2(point.x, point.y)
    }

    fn translate_point(point: Vector2, width: f32, height: f32, scale: f32) -> Vector2 {
        vec2(
            (width / 2.0) + point.x * (50.0 * scale),
            (height / 2.0) + point.y * (50.0 * scale),
        )
    }
}
