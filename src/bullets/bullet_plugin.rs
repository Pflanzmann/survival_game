use bevy::prelude::Plugin;
use crate::App;
use crate::bullets::bullet_hit_system::bullet_hit_system;
use crate::bullets::bullet_movement_system::bullet_movement_system;
use crate::bullets::modifications::modifications_plugin::ModificationsPlugin;
use crate::components::event_components::{BulletShotEvent, BulletEnemyCollisionEvent};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ModificationsPlugin)

            .add_event::<BulletShotEvent>()
            .add_event::<BulletEnemyCollisionEvent>()

            .add_system(bullet_movement_system)
            .add_system(bullet_hit_system);
    }
}