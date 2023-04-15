use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_egui::*;
use bevy_egui::egui::*;

use crate::MainMenuState;

#[derive(Default)]
pub struct LocalOptionsState {
    pub fullscreen_checkbox: bool,
}

pub fn show_options_system(
    asset_loader: Res<AssetServer>,
    mut egui_context: EguiContexts,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut option_state: Local<LocalOptionsState>,
    mut window_query: Query<&mut bevy::prelude::Window, With<PrimaryWindow>>,
) {
    let screen_size = egui_context.ctx_mut().screen_rect();
    let image_handle = asset_loader.load("sprites/ui/ítem_background.png");
    let background_image = egui_context.add_image(image_handle);

    let mut primary_window = match window_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

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

                ui.label(RichText::new("Options")
                    .heading()
                    .color(Color32::from_rgb(255, 255, 255))
                    .size(50.0)
                );
            });


            ui.vertical_centered(|ui| {
                ui.set_min_width(100.0);
                ui.set_max_width(400.0);
                ui.vertical_centered_justified(|ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(2)
                        .spacing([screen_size.width() * 0.1, screen_size.height() * 0.015])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("Fullscreen")
                                    .heading()
                                    .color(Color32::from_rgb(20, 20, 20))
                                    .monospace()
                                    .size(25.0)
                            );

                            let fullscreen_checkbox = ui.add(Checkbox::new(&mut option_state.fullscreen_checkbox, ""));
                            if fullscreen_checkbox.changed() {
                                if option_state.fullscreen_checkbox {
                                    primary_window.mode = WindowMode::Fullscreen;
                                } else {
                                    primary_window.mode = WindowMode::Windowed
                                }
                            }

                            ui.end_row();

                            ui.label(
                                RichText::new("Fullscreen")
                                    .heading()
                                    .color(Color32::from_rgb(20, 20, 20))
                                    .monospace()
                                    .size(25.0)
                            );

                            let fullscreen_checkbox = ui.add(Checkbox::new(&mut option_state.fullscreen_checkbox, ""));
                            if fullscreen_checkbox.changed() {
                                if option_state.fullscreen_checkbox {
                                    primary_window.mode = WindowMode::Fullscreen;
                                } else {
                                    primary_window.mode = WindowMode::Windowed
                                }
                            }

                            ui.end_row();

                            ui.label(
                                RichText::new("Fullscreen")
                                    .heading()
                                    .color(Color32::from_rgb(20, 20, 20))
                                    .monospace()
                                    .size(25.0)
                            );

                            let fullscreen_checkbox = ui.add(Checkbox::new(&mut option_state.fullscreen_checkbox, ""));
                            if fullscreen_checkbox.changed() {
                                if option_state.fullscreen_checkbox {
                                    primary_window.mode = WindowMode::Fullscreen;
                                } else {
                                    primary_window.mode = WindowMode::Windowed
                                }
                            }

                            ui.end_row();
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
