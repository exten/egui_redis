use eframe::{egui, epi};

use self::tree::Tree;

pub use super::component::tree;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state

pub struct RedisManageApp {
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
}

impl Default for RedisManageApp {
    fn default() -> Self {
        Self {
            label: "Test App".into(),
            value: 3.9,
        }
    }
}

impl epi::App for RedisManageApp {
    fn name(&self) -> &str {
        "this test app name"
    }

    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            println!("setup ...");
        }
    }

    fn save(&mut self, _storage: &mut dyn epi::Storage) {
        println!("save ...");
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { label, value } = self;
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("this is left panel");

            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    Tree::none().ui(ui);
                });
            // });
            egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 10.0;
                        ui.label("powered by ");
                        ui.hyperlink_to("google", "https://cao.com");
                        ui.label("and");
                        ui.hyperlink_to(" baidu", "https://baidu.com");
                    });
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(value, 0.0..=100.0).text("value"));
            // 显示数据
        });
    }
}
