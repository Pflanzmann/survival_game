use bevy::prelude::{Res};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Color32;
use crate::models::resources::ui_states::hud_state::HudState;


pub fn show_hud_system(
    mut egui_context: EguiContexts,
    state: Res<HudState>,
) {
    let mut images: Vec<egui::TextureId> = Vec::new();
    for handle in state.image_handles.iter() {
        let image = egui_context.add_image(handle.clone_weak());
        images.push(image);
    }

    egui::Area::new("Hud Area")
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::default())
        .show(egui_context.ctx_mut(), |ui| {
            ui.style_mut().visuals.code_bg_color = Color32::from_rgb(255, 2, 255);
            ui.set_max_size(egui::Vec2::new(500.0, 0.0));
            ui.horizontal_wrapped(|ui| {
                for image in images.iter() {
                    ui.image(*image, egui::Vec2::new(55.0, 55.0));
                }
            });
        });
}
