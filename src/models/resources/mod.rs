use bevy::prelude::{App, Plugin, Vec2};

use crate::models::resources::background_tiles_resource::BackgroundTilesResource;
use crate::models::resources::console_history::{ConsoleHistory, read_history_from_file};
use crate::models::resources::hit_box_quad_tree::HitBoxQuadTree;
use crate::models::resources::item_collision_quad_tree::ItemCollisionQuadTree;
use crate::models::resources::shop_customer::ShopCustomer;
use crate::models::resources::solid_body_quad_tree::SolidBodyQuadTree;
use crate::models::resources::spawn_phase_timer::SpawnPhaseTimer;
use crate::models::resources::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::resources::spawn_timer::SpawnIntervalTimer;
use crate::models::resources::state_resources::AppStateTrigger;

pub mod background_tiles_resource;
pub mod state_resources;
pub mod spawn_timer;
pub mod spawn_task_receiver;
pub mod console_history;
pub mod solid_body_quad_tree;
pub mod item_collision_quad_tree;
pub mod hit_box_quad_tree;
pub mod spawn_phase_timer;
pub mod shop_customer;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AppStateTrigger>()
            .init_resource::<SpawnIntervalTimer>()
            .init_resource::<SpawnPhaseTimer>()
            .init_resource::<SpawnTaskReceiver>()

            .insert_resource::<ConsoleHistory>(read_history_from_file())

            .insert_resource::<BackgroundTilesResource>(BackgroundTilesResource {
                current_origin: Vec2::new(-1.0, -1.0),
                tiles: Vec::new(),
            })

            .init_resource::<ItemCollisionQuadTree>()
            .init_resource::<SolidBodyQuadTree>()
            .init_resource::<HitBoxQuadTree>()
            .init_resource::<ShopCustomer>()
        ;
    }
}

