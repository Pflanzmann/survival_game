use bevy::prelude::{Deref, DerefMut, Entity, Resource};

use crate::models::collision::collider_type::ColliderType;
use crate::util::quad_tree::Quadtree;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SolidBodyQuadTree(pub Quadtree<SolidBodyData>);

#[derive(Copy, Clone)]
pub struct SolidBodyData {
    pub entity: Entity,
    pub collider_type: ColliderType,
    pub collision_weight: f32,
}