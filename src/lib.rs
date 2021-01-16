use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_webgl2::WebGL2Plugin;
use wasm_bindgen::prelude::*;

const BEVY_TEXTURE_ID: u64 = 0;

#[wasm_bindgen(start)]
pub fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WebGL2Plugin)
        .add_plugin(EguiPlugin)
        .add_startup_system(load_assets.system())
        .add_system(update_ui_scale_factor.system())
        .add_system(ui_example.system())
        .init_resource::<UiState>()
        .run();
}

#[derive(Default)]
struct UiState {
    label: String,
    value: f32,
    painting: Painting,
    inverted: bool,
}

fn load_assets(mut egui_context: ResMut<EguiContext>, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("icon.png");
    egui_context.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
}

fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}

fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    asset_server: Res<AssetServer>,
) {
    let ctx = &mut egui_context.ctx;

    let mut load = false;
    let mut remove = false;
    let mut invert = false;

    egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
        ui.heading("Side Panel");

        ui.horizontal(|ui| {
            ui.label("Write something: ");
            ui.text_edit_singleline(&mut ui_state.label);
        });

        ui.add(egui::Slider::f32(&mut ui_state.value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked {
            ui_state.value += 1.0;
        }

        ui.with_layout(egui::Layout::left_to_right(), |ui| {
            load = ui.button("Load").clicked;
            invert = ui.button("Invert").clicked;
            remove = ui.button("Remove").clicked;
        });

        ui.add(egui::widgets::Image::new(
            egui::TextureId::User(BEVY_TEXTURE_ID),
            [256.0, 256.0],
        ));

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add(egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"));
        });
    });

    egui::TopPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Quit").clicked {
                    std::process::exit(0);
                }
            });
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Egui Template");
        ui.hyperlink("https://github.com/emilk/egui_template");
        ui.add(egui::github_link_file_line!(
            "https://github.com/emilk/egui_template/blob/master/",
            "Direct link to source code."
        ));
        egui::warn_if_debug_build(ui);

        ui.separator();

        ui.heading("Central Panel");
        ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
        ui.label("It is often a great place for big things, like drawings:");

        ui.heading("Draw with your mouse to paint:");
        ui_state.painting.ui_control(ui);
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui_state.painting.ui_content(ui);
        });
    });

    egui::Window::new("Window").show(ctx, |ui| {
        ui.label("Windows can be moved by dragging them.");
        ui.label("They are automatically sized based on contents.");
        ui.label("You can turn on resizing and scrolling if you like.");
        ui.label("You would normally chose either panels OR windows.");
    });

    if invert {
        ui_state.inverted = !ui_state.inverted;
    }
    if load || invert {
        let texture_handle = if ui_state.inverted {
            asset_server.load("icon_inverted.png")
        } else {
            asset_server.load("icon.png")
        };
        egui_context.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
    }
    if remove {
        egui_context.remove_egui_texture(BEVY_TEXTURE_ID);
    }
}

struct Painting {
    lines: Vec<Vec<egui::Vec2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            self.stroke.ui(ui, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked {
                self.lines.clear();
            }
        })
        .1
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());
        let rect = response.rect;

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if response.active {
            if let Some(mouse_pos) = ui.input().mouse.pos {
                let canvas_pos = mouse_pos - rect.min;
                if current_line.last() != Some(&canvas_pos) {
                    current_line.push(canvas_pos);
                }
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
        }

        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<egui::Pos2> = line.iter().map(|p| rect.min + *p).collect();
                painter.add(egui::PaintCmd::line(points, self.stroke));
            }
        }

        response
    }
}
