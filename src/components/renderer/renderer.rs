use egui::Color32;

use crate::{app::Projection, GomuOriApp};

use super::{
    vector2::{vec2, Vector2},
    vector3::{vec3, Vector3},
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

        if let Some(points) = &state.shape {
            points
                .into_iter()
                .map(|p| Self::rotate_point(*p, Angle::X, state.rotation.x))
                .map(|p| Self::rotate_point(p, Angle::Y, state.rotation.y))
                .map(|p| Self::rotate_point(p, Angle::Z, state.rotation.z))
                .map(|p| match state.projection {
                    Projection::Orthographic { scale } => {
                        Self::project_point_orthographic(p, scale)
                    }
                    Projection::Perspective {
                        camera_position,
                        fov,
                    } => Self::project_point_perspective(p, camera_position, fov),
                })
                .map(|p| Self::center_point(p, width, height))
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
        if !(x < 0.0
            || y < 0.0
            || x > (self.surface.width - 1.0)
            || y > (self.surface.height - 1.0))
        {
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

    fn project_point_orthographic(point: Vector3, scale: f32) -> Vector2 {
        vec2(point.x * 30.0 * scale, point.y * 30.0 * scale)
    }

    fn project_point_perspective(point: Vector3, camera_pos: Vector3, fov: f32) -> Vector2 {
        let x = (point.x * fov) / (point.z + camera_pos.z); // This 1.0 is the distance between the camera and the
                                                            // projection plane
        let y = (point.y * fov) / (point.z + camera_pos.z);
        vec2(x, y)
    }

    fn center_point(point: Vector2, width: f32, height: f32) -> Vector2 {
        vec2((width / 2.0) + point.x, (height / 2.0) + point.y)
    }

    fn rotate_point(point: Vector3, angle: Angle, rotation: f32) -> Vector3 {
        match angle {
            Angle::X => vec3(
                point.x,
                point.y * rotation.cos() - point.z * rotation.sin(),
                point.y * rotation.sin() + point.z * rotation.cos(),
            ),
            Angle::Y => vec3(
                point.x * rotation.cos() - point.z * rotation.sin(),
                point.y,
                point.x * rotation.sin() + point.z * rotation.cos(),
            ),
            Angle::Z => vec3(
                point.x * rotation.cos() - point.y * rotation.sin(),
                point.x * rotation.sin() + point.y * rotation.cos(),
                point.z,
            ),
        }
    }
}

enum Angle {
    X,
    Y,
    Z,
}
