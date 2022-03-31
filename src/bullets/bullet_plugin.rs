use bevy::prelude::Plugin;
use crate::App;
use crate::bullets::bullet_movement_system::bullet_movement_system;
use crate::bullets::modifications::modifications_plugin::ModificationsPlugin;
use crate::components::event_components::BulletShotEvent;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ModificationsPlugin)

            .add_event::<BulletShotEvent>()

            .add_system(bullet_movement_system);
    }
}