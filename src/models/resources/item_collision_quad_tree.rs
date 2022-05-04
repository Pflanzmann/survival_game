use bevy::prelude::{Deref, DerefMut};

use crate::util::quad_tree::Quadtree;

#[derive(Default, Deref, DerefMut)]
pub struct ItemCollisionQuadTree(pub Quadtree);
