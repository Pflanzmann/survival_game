use bevy::prelude::{EventReader, Res, Sprite, SpriteBundle, Vec2, With};

use crate::{Commands, Query, Transform, UnitSize};
use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::entities::collider::collider::Collider;
use crate::entities::events::enemy_died_event::EnemyDiedEvent;
use crate::entities::item_components::Item;
use crate::entities::unit_stats_components::Enemy;

pub fn basic_drop_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<EnemyDiedEvent>,
    texture_handles: Res<TextureHandles>,
    item_handles: Res<ItemConfigHandles>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for event in enemy_died_event.iter() {
        let enemy_position = match enemy_query.get(event.enemy_entity) {
            Ok(transform) => transform.translation,
            Err(_) => continue,
        };

        let mut drop_translation = enemy_position.clone();
        drop_translation.z += 1.0;

        commands.spawn_bundle(
            SpriteBundle {
                transform: Transform::from_translation(drop_translation),
                texture: texture_handles.basic_drop_asset_handler.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(item_handles.coin.sprite_custom_size_x, item_handles.coin.sprite_custom_size_y)),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
            .insert(Item)
            .insert(Collider)
            .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) });
    }
}