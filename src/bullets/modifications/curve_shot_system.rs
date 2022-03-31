use bevy::app::EventReader;
use bevy::prelude::{Commands, Query, With};
use rand::random;

use crate::components::event_components::BulletShotEvent;
use crate::components::modification_components::CurveShot;
use crate::FacingDirection;

pub fn apply_curved_shot_system(
    mut commands: Commands,
    mut bullet_shot_event: EventReader<BulletShotEvent>,
) {
    for event in bullet_shot_event.iter() {
        commands.entity(event.entity).insert(CurveShot { curve_left: random() });
    }
}

pub fn curve_shot_system(
    mut bullet_query: Query<(&mut FacingDirection, &CurveShot)>,
) {
    for (mut direction, curve_shot) in bullet_query.iter_mut() {
        let angle: f32 = if curve_shot.curve_left { 0.005 } else { -0.005 };
        let x = direction.facing_direction.x;
        let y = direction.facing_direction.y;

        direction.facing_direction.x = x * angle.cos() - y * angle.sin();
        direction.facing_direction.y = x * angle.sin() + y * angle.cos();

        direction.facing_direction = direction.facing_direction.normalize();
    }
}