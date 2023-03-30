use bevy::prelude::*;
use bevy_egui::*;

use crate::AppState;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::player::Player;
use crate::models::resources::ui_states::shop_state::ShopState;

pub fn show_shop_system(
    mut egui_context: EguiContexts,
    state: Res<ShopState>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut get_ability_event: EventWriter<ApplyModToTargetEvent>,
    player_query: Query<Entity, With<Player>>,
) {
    let screen_size = egui_context.ctx_mut().screen_rect();

    let shop_width = screen_size.width() * 0.5;
    let shop_height = screen_size.height() * 0.7;
    let text_space_size = screen_size.width() * 0.03;
    let shop_mod_size = shop_height * 0.3;

    egui::Window::new("Shop window")
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
        .show(egui_context.ctx_mut(), |ui| {
// ui.put(egui::Rect::from([Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)]), egui::Image::new(background_image, egui::Vec2::new(shop_width, shop_height)));

            ui.set_max_size(egui::Vec2::new(shop_width, shop_height));

            ui.vertical_centered(|ui| {
                for chosen_mod in state.chosen_mod.iter() {
                    ui.horizontal_wrapped(|ui| {
                        ui.image(chosen_mod.texture_id, egui::Vec2::new(shop_mod_size, shop_mod_size));

                        ui.add_space(text_space_size);
                        ui.vertical(|ui| {
                            ui.label(&chosen_mod.title);

                            ui.add_space(text_space_size);

                            if ui.button("Get Ability").clicked() {
                                for entity in player_query.iter() {
                                    get_ability_event.send(ApplyModToTargetEvent { target_entity: entity, mod_entity: chosen_mod.entity });
                                    next_app_state.set(AppState::InGame);
                                }
                            }
                        });

                        ui.add_space(text_space_size);

                        ui.horizontal_wrapped(|ui| {
                            ui.label(&chosen_mod.description);
                        });
                    });
                };
            });
        });
}