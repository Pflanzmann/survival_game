use bevy::prelude::*;

use crate::models::resources::solid_body_quad_tree::SolidBodyQuadTree;
use crate::SpriteLayer;

#[derive(Component)]
pub struct TreeArea;

pub fn show_quad_tree_system(
    mut commands: Commands,
    area_query: Query<Entity, With<TreeArea>>,
    quad_tree: Res<SolidBodyQuadTree>,
) {
    if quad_tree.is_changed() {
        for entity in area_query.iter() {
            commands.entity(entity).despawn();
        }

        let mut output: Vec<(Vec2, f32, f32, usize)> = Vec::new();
        quad_tree.get_all_squares(&mut output);

        for (pos, width, height, layer) in output {
            let color = 0.1 * layer as f32;
            commands.spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::from([color, color, color, color]),
                        custom_size: Some(Vec2::new(width * 0.99, height * 0.99)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(pos.x, pos.y, SpriteLayer::AirLevel.get_layer_z()),
                    ..Default::default()
                }
            ).insert(TreeArea);
        }
    }
}