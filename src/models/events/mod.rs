use bevy::prelude::{App, Plugin};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::events::bullet_enemy_collision_event::BulletEnemyCollisionEvent;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::events::damaged_event::DamagedEvent;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::events::player_died_event::PlayerDiedEvent;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;
use crate::models::events::remove_mod_from_target_event::RemoveModFromTargetEvent;

pub mod bullet_shot_event;
pub mod enemy_died_event;
pub mod bullet_enemy_collision_event;
pub mod bullet_stopped_event;
pub mod item_collision_event;
pub mod player_enemy_collision_event;
pub mod player_died_event;
pub mod apply_mod_to_target_event;
pub mod damaged_event;
pub mod remove_mod_from_target_event;

/// This plugin defines events and their contents for several occasions
///
/// [ apply_mod_to_target ] is used to connect a unit and a mod entity in order to apply changes to the unit
///
/// [ bullet_enemy_collision_event ] reacts when an enemy is hit by a projectile. Applies damage to the enemy and
/// possible changes to the projectile.
///
/// [ bullet_shot_event ] triggers adding modifications to newly spawned projectiles
///
/// [ bullet_stopped_event ] reaction to despawning projectiles to trigger effects on that
///
/// [ enemy_died_event ] reaction to enemies dying to calculate item drops or other things
///
/// [ item_collision_event ] triggers reaction to the player collecting an item
///
/// [ player_died_event ] reacts to the player dying
///
/// [ player_enemy_collision_event ] triggers reaction to the player being "hit" by an enemy

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
            .add_event::<PlayerDiedEvent>()
            .add_event::<ApplyModToTargetEvent>()
            .add_event::<RemoveModFromTargetEvent>()
            .add_event::<DamagedEvent>()
        ;
    }
}