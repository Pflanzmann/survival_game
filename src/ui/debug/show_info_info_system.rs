use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::models::resources::ui_states::info_window_state::InfoWindowState;

pub fn show_info_window_system(
    mut egui_context: EguiContexts,
    mut state: ResMut<InfoWindowState>,
) {
    egui::Window::new("Info")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-10.0, -10.0))
        .fixed_size(egui::Vec2::new(200.0, 200.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(200.0, 0.0));

            state.infos.keys().for_each(|key| {
                ui.horizontal(|ui| {
                    ui.label(key.to_string() + ": ");
                    ui.label(state.infos[key].to_string());
                });
            });
        });
}
