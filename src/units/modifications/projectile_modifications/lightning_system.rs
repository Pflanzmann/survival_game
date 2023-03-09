use bevy::core::Name;
use bevy::prelude::{AssetServer, Commands, Entity, EventReader, EventWriter, GlobalTransform, Quat, Query, Res, Sprite, SpriteBundle, Transform, Vec2};
use bevy::sprite::Anchor;
use rand::random;

use crate::models::animation::fade_animation::FadeAnimation;
use crate::models::collision::collider_type::ColliderType;
use crate::models::damaged_entities::DamagedEntities;
use crate::models::events::damaged_event::DamagedEvent;
use crate::models::events::enemy_collision_event::EnemyCollisionEvent;
use crate::models::modifications::lightning::Lightning;
use crate::models::resources::collision::hit_box_quad_tree::{HitBoxData, HitBoxQuadTree};
use crate::models::time_alive::TimeAlive;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::SpriteLayer;
use crate::util::quad_tree::QuadData;

pub fn lightning_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    quad_tree: Res<HitBoxQuadTree>,
    mut enemy_hit_event: EventWriter<EnemyCollisionEvent>,
    mut damaged_events: EventReader<DamagedEvent>,
    source_query: Query<(&GlobalTransform, &Damage, &Lightning)>,
) {
    for event in damaged_events.iter() {
        let (transform, damage, lightning) = match source_query.get(event.source_entity) {
            Ok(source) => source,
            Err(_) => continue,
        };

        let chance = random::<f32>();
        if chance > lightning.chance {
            continue;
        }

        let size = ColliderType::Circle(lightning.radius);

        let mut position = transform.translation().truncate();

        let mut already_hit_targets: Vec<Entity> = vec![event.target_entity];

        'outer: for _ in 0..lightning.jump_count {
            let mut check_entity_list: Vec<QuadData<HitBoxData>> = Vec::new();
            quad_tree.query_entities(
                &mut check_entity_list,
                &position,
                &Vec2::new(lightning.radius / 2.0, lightning.radius / 2.0),
            );

            for quad_data in check_entity_list.iter() {
                if already_hit_targets.contains(&quad_data.data.entity) {
                    continue;
                }

                if size.is_colliding(&position, &quad_data.data.collider_type, &quad_data.position) {
                    let mut lightning_transform = Transform::from_translation(position.extend(SpriteLayer::AirLevel.get_layer_z()));

                    let direction = (quad_data.position - position).normalize();
                    lightning_transform.rotation = Quat::from_rotation_arc_2d(Vec2::new(0.0, 1.0), direction);

                    let distance = position.distance(quad_data.position);

                    let lightning_entity = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(lightning.lightning_width, distance)),
                            anchor: Anchor::BottomCenter,
                            ..Default::default()
                        },
                        texture: asset_server.load(&lightning.lightning_sprite_path),
                        transform: lightning_transform,
                        ..Default::default()
                    })
                        .insert(Name::new("Lightning"))
                        .insert(TimeAlive { time_alive: lightning.sprite_time_alive })
                        .insert(DamagedEntities::default())
                        .insert(FadeAnimation { fade_time: -lightning.sprite_time_alive })
                        .insert(Damage::new(damage.get_total_amount())).id();

                    position = quad_data.position;
                    already_hit_targets.push(quad_data.data.entity);

                    enemy_hit_event.send(EnemyCollisionEvent { source_entity: lightning_entity, target_entity: quad_data.data.entity });
                    continue 'outer;
                }
            }

            break 'outer;
        }
    }
}