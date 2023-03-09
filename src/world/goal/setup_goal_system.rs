use bevy::prelude::{AssetServer, BuildChildren, Color, Commands, Name, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3};

use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::layerable::Layerable;
use crate::models::world::goal::Goal;
use crate::models::world::goal_activation_progress::GoalActivationProgress;
use crate::SpriteLayer;

pub fn setup_goal_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let position = Vec3::new(-2000.0, 0.0, SpriteLayer::LowGroundLevel.get_layer_z());

    commands.spawn(
        SpriteBundle {
            transform: Transform::from_translation(position),
            texture: asset_server.load("sprites/mailbox.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 206.0)),
                ..Default::default()
            },
            ..Default::default()
        }
    )
        .insert(Name::new("Goal"))
        .insert(Layerable::new(SpriteLayer::LowGroundLevel.get_layer_z()))
        .insert(HitBoxCollider { collider_type: ColliderType::Rectangle(Vec2::new(700.0, 700.0)) })
        .insert(GoalActivationProgress { progress: 0.0 })
        .insert(Goal)
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, -70.0)),
                texture: asset_server.load("sprites/collider_circle.png"),
                sprite: Sprite {
                    color: Color::from([0.0, 0.0, 0.4, 0.4]),
                    custom_size: Some(Vec2::new(700.0, 700.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
    ;
}