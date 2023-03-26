use bevy::prelude::{App, FixedTime, Plugin, Vec2};

use collision::hit_box_quad_tree::HitBoxQuadTree;
use collision::item_collision_quad_tree::ItemCollisionQuadTree;
use collision::solid_body_quad_tree::SolidBodyQuadTree;
use world::background_tiles_resource::BackgroundTilesResource;
use world::spawn_phase_timer::SpawnPhaseTimer;
use world::spawn_task_receiver::SpawnTaskReceiver;
use world::spawn_timer::SpawnIntervalTimer;

use crate::models::resources::console_history::{ConsoleHistory, read_history_from_file};
use crate::models::resources::shop_customer::ShopCustomer;
use crate::models::resources::ui_states::hud_state::HudState;
use crate::models::resources::world::game_time::GameTime;

pub mod console_history;
pub mod shop_customer;
pub mod world;
pub mod collision;
pub mod ui_states;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ItemCollisionQuadTree>()
            .init_resource::<SolidBodyQuadTree>()
            .init_resource::<HitBoxQuadTree>()

            .insert_resource::<BackgroundTilesResource>(BackgroundTilesResource { current_origin: Vec2::new(-1.0, -1.0), tiles: Vec::new() })
            .init_resource::<SpawnIntervalTimer>()
            .init_resource::<SpawnPhaseTimer>()
            .init_resource::<SpawnTaskReceiver>()
            .init_resource::<GameTime>()

            .init_resource::<ShopCustomer>()
            .insert_resource::<ConsoleHistory>(read_history_from_file())

            .init_resource::<HudState>()

            .insert_resource(FixedTime::new_from_secs(0.1))
        ;
    }
}

