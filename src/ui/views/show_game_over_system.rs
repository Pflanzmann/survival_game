use bevy::ecs::event::Events;
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn show_game_over_system(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut egui_context: EguiContexts,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.allocate_space(egui::Vec2::new(0.0, 100.0));

            ui.set_min_size(egui::Vec2::new(0.0, 100.0));

            ui.heading("You dieded");
        });

        ui.vertical_centered(|ui| {
            ui.set_min_width(400.0);
            ui.set_max_width(400.0);
            ui.vertical_centered_justified(|ui| {
                if ui.add_sized([100.0, 80.0], egui::Button::new("Rage Quit")).clicked() {
                    app_exit_events.send(bevy::app::AppExit);
                };
            });
        });
    });
}