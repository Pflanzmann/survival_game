use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::*;

use crate::AppState;

pub fn show_main_menu_system(
    asset_loader: Res<AssetServer>,
    mut egui_context: EguiContexts,
    mut state_trigger: ResMut<NextState<AppState>>,
) {
    let screen_size = egui_context.ctx_mut().screen_rect();
    let temp = asset_loader.load("sprites/ui/ítem_background.png");
    let background_image = egui_context.add_image(temp);

    //background color
    let background_color = egui::containers::Frame {
        fill: egui::Color32::from_rgb(241, 233, 218),
        ..Default::default()
    };
    let frame = egui::containers::Frame {
        fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
        ..Default::default()
    };


    //main window
    egui::CentralPanel::default()
        .frame(background_color)
        .show(egui_context.ctx_mut(), |ui| {
            ui.image(background_image, vec2(screen_size.width(), screen_size.height()));
        });

    egui::CentralPanel::default()
        .frame(frame)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.allocate_space(egui::Vec2::new(0.0, 100.0));

                ui.set_min_size(egui::Vec2::new(0.0, 100.0));

                ui.label(egui::RichText::new("Atomic Under Paper Lord Chess")
                    .heading()
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .size(50.0)
                );
            });

            ui.vertical_centered(|ui| {
                ui.set_min_width(100.0);
                ui.set_max_width(400.0);
                ui.vertical_centered_justified(|ui| {
                    if ui.add(egui::Button::new(egui::RichText::new("Start Game")
                        .color(egui::Color32::from_rgb(0, 0, 0))
                        .size(30.0))
                    ).clicked() {
                        state_trigger.set(AppState::InGame);
                    };
                });
            });
        });
}
