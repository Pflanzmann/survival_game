use bevy::prelude::{Color, Commands, Entity, Query, Res, Sprite, TextureAtlasSprite, Time};

use crate::models::damaged_effect::DamagedEffect;

pub fn apply_hit_effect_sprite_system(
    time: Res<Time>,
    mut hit_effect_query: Query<(Entity, &mut Sprite, &mut DamagedEffect)>,
    mut commands: Commands,
) {
    for (entity, mut handle, mut damaged_effect) in hit_effect_query.iter_mut() {
        handle.color = Color::rgb(1.0, 0.0, 0.0);
        damaged_effect.timer += time.delta().as_secs_f32();

        if damaged_effect.timer >= 0.15 {
            commands.entity(entity).remove::<DamagedEffect>();
            handle.color = Color::rgb(1.0, 1.0, 1.0);
        }
    }
}

pub fn apply_hit_effect_sprite_atlas_system(
    time: Res<Time>,
    mut hit_effect_query: Query<(Entity, &mut TextureAtlasSprite, &mut DamagedEffect)>,
    mut commands: Commands,
) {
    for (entity, mut handle, mut damaged_effect) in hit_effect_query.iter_mut() {
        handle.color = Color::rgb(1.0, 0.0, 0.0);
        damaged_effect.timer += time.delta().as_secs_f32();

        if damaged_effect.timer >= 0.15 {
            commands.entity(entity).remove::<DamagedEffect>();
            handle.color = Color::rgb(1.0, 1.0, 1.0);
        }
    }
}
