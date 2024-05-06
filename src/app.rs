use std::f32::consts::PI;

use egui::{epaint::text, pos2};

use crate::components::renderer::{
    vector3::{vec3, Vector3},
    Renderer,
};

#[derive(PartialEq)]
pub enum Projection {
    Orthographic { scale: f32 },
    Perspective { camera_position: Vector3, fov: f32 },
}
impl Projection {
    fn default_orthographic() -> Self {
        Projection::Orthographic { scale: 1.0 }
    }

    fn default_perspective() -> Self {
        Projection::Perspective {
            camera_position: vec3(0.0, 0.0, -1.0),
            fov: 120.0,
        }
    }
}

pub struct GomuOriApp {
    pub projection: Projection,
    pub shape: Option<Vec<Vector3>>,
    pub rotation: Vector3,
}

impl Default for GomuOriApp {
    fn default() -> Self {
        Self {
            rotation: vec3(0.0, 0.0, 0.0),
            projection: Projection::default_perspective(),
            shape: Some(cube()),
        }
    }
}

impl GomuOriApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }

    fn projection_is_perspective(&self) -> bool {
        matches!(self.projection, Projection::Perspective { .. })
    }

    fn projection_is_orthographic(&self) -> bool {
        matches!(self.projection, Projection::Orthographic { .. })
    }
}

impl eframe::App for GomuOriApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                Renderer::create(ui).draw(&self);
                egui::Area::new("Rendere Settings".into())
                    .fixed_pos(egui::pos2(5.0, 5.0))
                    .show(ctx, |ui| {
                        // egui::Frame::popup(ui.style())
                        // .stroke(Stroke::NONE)
                        // .show(ui, |ui| {
                        egui::CollapsingHeader::new("Renderer Settings").show(ui, |ui| {
                            egui::CollapsingHeader::new("Projection").show(ui, |ui| {
                                if ui
                                    .radio(self.projection_is_orthographic(), "Orthographic")
                                    .clicked()
                                {
                                    self.projection = Projection::default_orthographic();
                                }
                                if ui
                                    .radio(self.projection_is_perspective(), "Perspective")
                                    .clicked()
                                {
                                    self.projection = Projection::default_perspective();
                                }
                            });

                            match &mut self.projection {
                                Projection::Orthographic { scale } => {
                                    ui.add(
                                        egui::Slider::new(scale, 1.0..=15.0).text("Scale factor"),
                                    );
                                }
                                Projection::Perspective {
                                    camera_position,
                                    fov,
                                } => {
                                    ui.add(
                                        egui::Slider::new(fov, 120.0..=240.0)
                                            .text("FOV (Field of view)"),
                                    );
                                    ui.add(
                                        egui::Slider::new(&mut camera_position.z, -1.0..=-15.0)
                                            .text("Camera Z"),
                                    );
                                }
                            }

                            egui::CollapsingHeader::new("Object rotation")
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::Slider::new(
                                            &mut self.rotation.x,
                                            0.0..=std::f32::consts::PI * 2.0,
                                        )
                                        .text("X axis"),
                                    );
                                    ui.add(
                                        egui::Slider::new(
                                            &mut self.rotation.y,
                                            0.0..=std::f32::consts::PI * 2.0,
                                        )
                                        .text("Y axis"),
                                    );
                                    ui.add(
                                        egui::Slider::new(
                                            &mut self.rotation.z,
                                            0.0..=std::f32::consts::PI * 2.0,
                                        )
                                        .text("Z axis"),
                                    );
                                });
                        })
                    })
            });
    }
}

fn cube() -> Vec<Vector3> {
    let mut shape = Vec::with_capacity(9 * 9 * 9);

    let inc = 0.25;
    let mut x = -1.0;
    let mut y = -1.0;
    let mut z = -1.0;

    while x <= 1.0 {
        while y <= 1.0 {
            while z <= 1.0 {
                shape.push(vec3(x, y, z));
                z += inc;
            }
            z = -1.0;
            y += inc;
        }
        y = -1.0;
        x += inc;
    }

    shape
}
