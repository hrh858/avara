use std::ops::RangeInclusive;

use egui::Stroke;

use crate::components::renderer::{
    vector3::{vec3, Vector3},
    Renderer,
};

#[derive(PartialEq)]
pub enum Projection {
    Orthographic,
    Perspective,
}

pub struct GomuOriApp {
    pub scale: f32,
    pub projection: Projection,
    pub shape: Option<Vec<Vector3>>,
}

impl Default for GomuOriApp {
    fn default() -> Self {
        Self {
            projection: Projection::Orthographic,
            scale: 1.0,
            // shape: None,
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
}

impl eframe::App for GomuOriApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         // NOTE: no File->Quit on web pages!
        //         let is_web = cfg!(target_arch = "wasm32");
        //         if !is_web {
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }
        //         egui::widgets::global_dark_light_mode_buttons(ui);
        //         let scale_slider =
        //             egui::widgets::Slider::new(&mut self.scale, RangeInclusive::new(1.0, 15.0));
        //         ui.radio_value(
        //             &mut self.projection,
        //             Projection::Orthographic,
        //             "Orthographic",
        //         );
        //         ui.radio_value(&mut self.projection, Projection::Perspective, "Perspective");
        //         ui.add(scale_slider);
        //     });
        // });

        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                // let _res = ui.centered_and_justIfied(|ui| Renderer::create(ui).draw(&self));
                Renderer::create(ui).draw(&self);
                egui::Area::new("Rendere Settings".into())
                    .fixed_pos(egui::pos2(5.0, 5.0))
                    .show(ctx, |ui| {
                        // egui::Frame::popup(ui.style())
                        // .stroke(Stroke::NONE)
                        // .show(ui, |ui| {
                        egui::CollapsingHeader::new("Renderer Settings").show(ui, |ui| {
                            egui::CollapsingHeader::new("Projection").show(ui, |ui| {
                                ui.radio_value(
                                    &mut self.projection,
                                    Projection::Orthographic,
                                    "Orthographic",
                                );
                                ui.radio_value(
                                    &mut self.projection,
                                    Projection::Perspective,
                                    "Perspective",
                                );
                            });
                            ui.add(
                                egui::Slider::new(&mut self.scale, 1.0..=15.0).text("Scale factor"),
                            )
                        })
                        // });
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
