use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::*;

use crate::{AppState, MainMenuState};

pub fn show_title_screen_system(
    asset_loader: Res<AssetServer>,
    mut egui_context: EguiContexts,
    keys: Res<Input<KeyCode>>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    let screen_size = egui_context.ctx_mut().screen_rect();
    let image_handle = asset_loader.load("images/title_screen.png");
    let background_image = egui_context.add_image(image_handle);

    //background color
    let background_color = Frame {
        fill: Color32::from_rgb(241, 233, 218),
        ..Default::default()
    };
    let frame = Frame {
        fill: Color32::from_rgba_premultiplied(0, 0, 0, 0),
        ..Default::default()
    };


    //entry background
    CentralPanel::default()
        .frame(background_color)
        .show(egui_context.ctx_mut(), |ui| {
            ui.image(background_image, vec2(screen_size.width(), screen_size.height()));
        });

    CentralPanel::default()
        .frame(frame)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(screen_size.height() * 0.847);

                ui.label(RichText::new("Press space to continue!")
                    .heading()
                    .color(Color32::from_rgb(20, 20, 20))
                    .monospace()
                    .size(25.0)
                );
            });
        });

    if keys.pressed(KeyCode::Space) {
        next_menu_state.set(MainMenuState::MainMenu);
    }
}
