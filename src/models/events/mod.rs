use bevy::prelude::{App, Plugin};

use crate::models::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::events::player_died_event::PlayerDiedEvent;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;

pub mod bullet_shot_event;
pub mod enemy_died_event;
pub mod bullet_enemy_collision_event;
pub mod bullet_stopped_event;
pub mod item_collision_event;
pub mod player_enemy_collision_event;
pub mod player_died_event;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnemyDiedEvent>()
            .add_event::<ItemCollisionEvent>()
            .add_event::<BulletShotEvent>()
            .add_event::<BulletEnemyCollisionEvent>()
            .add_event::<BulletStoppedEvent>()
            .add_event::<PlayerEnemyCollisionEvent>()
            .add_event::<PlayerDiedEvent>();
    }
}