use bevy::prelude::{EventReader, Res, Sprite, SpriteBundle, Vec2, With};
use rand::random;

use crate::{Commands, Query, Transform};
use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::collider::collider::Collider;
use crate::models::events::enemy_died_event::EnemyDiedEvent;
use crate::models::item_components::{Coin, Heal, Item, Shop};
use crate::models::sprite_layer::SpriteLayer;
use crate::models::unit_stats_components::{Enemy, UnitSize};

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

        let mut drop_translation = enemy_position;
        drop_translation.z = SpriteLayer::LowGroundLevel.get_layer_z();

        let random = random::<f32>() * 100.0;

        if (0.0..20.0).contains(&random) {
            commands.spawn_bundle(
                SpriteBundle {
                    transform: Transform::from_translation(drop_translation),
                    texture: texture_handles.coin_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(item_handles.coin.sprite_custom_size_x, item_handles.coin.sprite_custom_size_y)),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            )
                .insert(Item)
                .insert(Collider)
                .insert(Coin)
                .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) });
        }

        if (20.0..40.0).contains(&random) {
            commands.spawn_bundle(
                SpriteBundle {
                    transform: Transform::from_translation(drop_translation),
                    texture: texture_handles.hot_dog_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(item_handles.hot_dog.sprite_custom_size_x, item_handles.hot_dog.sprite_custom_size_y)),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            )
                .insert(Item)
                .insert(Collider)
                .insert(Heal { amount: item_handles.hot_dog.heal_amount })
                .insert(UnitSize { collider_size: Vec2::new(item_handles.hot_dog.sprite_custom_size_x, item_handles.hot_dog.sprite_custom_size_y) });
        }

        if (40.0..100.0).contains(&random) {
            commands.spawn_bundle(
                SpriteBundle {
                    transform: Transform::from_translation(drop_translation),
                    texture: texture_handles.kiste_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(item_handles.kiste.sprite_custom_size_x, item_handles.kiste.sprite_custom_size_y)),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            )
                .insert(Item)
                .insert(Collider)
                .insert(Shop)
                .insert(UnitSize { collider_size: Vec2::new(item_handles.kiste.sprite_custom_size_x, item_handles.kiste.sprite_custom_size_y) });
        }
    }
}