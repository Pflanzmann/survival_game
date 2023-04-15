use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::*;

use crate::{AppState, MainMenuState};
use crate::assets_handling::preload_stage_spawn_system::StageList;
use crate::models::resources::world::active_stage::ActiveStage;

pub fn show_choose_stage_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut egui_context: EguiContexts,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    stage_list: Res<StageList>,
) {
    let screen_size = egui_context.ctx_mut().screen_rect();
    let image_handle = asset_loader.load("sprites/ui/ítem_background.png");
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

    //main window background
    CentralPanel::default()
        .frame(background_color)
        .show(egui_context.ctx_mut(), |ui| {
            ui.image(background_image, vec2(screen_size.width(), screen_size.height()));
        });

    CentralPanel::default()
        .frame(frame)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.allocate_space(egui::Vec2::new(0.0, 100.0));

                ui.set_min_size(egui::Vec2::new(0.0, 100.0));

                ui.label(RichText::new("Choose stage")
                    .heading()
                    .color(Color32::from_rgb(255, 255, 255))
                    .size(50.0)
                );
            });


            ui.vertical_centered(|ui| {
                ui.set_min_width(100.0);
                ui.set_max_width(400.0);
                ui.vertical_centered_justified(|ui| {
                    Grid::new("my_grid")
                        .num_columns(2)
                        .spacing([screen_size.width() * 0.1, screen_size.height() * 0.015])
                        .striped(true)
                        .show(ui, |ui| {
                            for stage in stage_list.stages.iter() {
                                ui.label(
                                    RichText::new(&stage.name)
                                        .heading()
                                        .color(Color32::from_rgb(20, 20, 20))
                                        .monospace()
                                        .size(25.0)
                                );

                                if ui.add(egui::Button::new(
                                    RichText::new("Play")
                                        .color(Color32::from_rgb(255, 255, 255))
                                        .monospace()
                                        .size(25.0))
                                ).clicked() {
                                    commands.insert_resource(ActiveStage::from_stage(&stage));

                                    next_app_state.set(AppState::InGame);
                                    next_menu_state.set(MainMenuState::MainMenu);
                                };

                                ui.end_row();
                            }
                        });

                    ui.add_space(screen_size.height() * 0.1);

                    if ui.add(egui::Button::new(
                        RichText::new("Back to the Main Menu")
                            .color(Color32::from_rgb(255, 255, 255))
                            .monospace()
                            .size(25.0))
                    ).clicked() {
                        next_menu_state.set(MainMenuState::MainMenu)
                    };
                });
            });
        });
}
