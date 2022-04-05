use bevy::prelude::{App, Plugin};

use crate::entities::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::entities::events::bullet_shot_event::BulletShotEvent;
use crate::entities::events::bullet_stopped_event::BulletStoppedEvent;
use crate::entities::events::enemy_died_event::EnemyDiedEvent;
use crate::entities::events::item_collision_event::ItemCollisionEvent;

pub mod bullet_shot_event;
pub mod enemy_died_event;
pub mod bullet_enemy_collision_event;
pub mod bullet_stopped_event;
pub mod item_collision_event;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_event::<EnemyDiedEvent>()
            .add_event::<ItemCollisionEvent>()
            .add_event::<BulletShotEvent>()
            .add_event::<BulletEnemyCollisionEvent>()
            .add_event::<BulletStoppedEvent>();
    }
}