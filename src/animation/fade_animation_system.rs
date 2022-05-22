use bevy::prelude::{Commands, Entity, Query, Res, Sprite, Time};

use crate::models::animation::fade_animation::FadeAnimation;

pub fn fade_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut sprite_query: Query<(Entity, &mut Sprite, &FadeAnimation)>,
) {
    for (entity, mut sprite, fade_animation) in sprite_query.iter_mut() {
        let new_alpha = sprite.color.a() + (time.delta_seconds() / fade_animation.fade_time);

        sprite.color.set_a(new_alpha);

        if new_alpha > 1.0 {
            sprite.color.set_a(1.0);
            commands.entity(entity).remove::<FadeAnimation>();
            return;
        }

        if new_alpha < 0.0 {
            sprite.color.set_a(0.0);
            commands.entity(entity).remove::<FadeAnimation>();
            return;
        }
    }
}