use bevy::prelude::{Deref, DerefMut, Entity};

use crate::models::collision::collider_type::ColliderType;
use crate::util::quad_tree::Quadtree;

#[derive(Default, Deref, DerefMut)]
pub struct HitBoxQuadTree(pub Quadtree<HitBoxData>);

#[derive(Copy, Clone)]
pub struct HitBoxData {
    pub entity: Entity,
    pub collider_type: ColliderType,
}