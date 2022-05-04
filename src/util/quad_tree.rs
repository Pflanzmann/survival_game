use bevy::prelude::{Entity, Vec2, Vec3};

use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_type::ColliderType::{Circle, Rectangle};

#[derive(Copy, Clone)]
pub struct QuadData {
    pub entity: Entity,
    pub position: Vec3,
    pub collider_type: ColliderType,
    pub collision_weight: f32,
}

pub struct Quadtree {
    width: f32,
    height: f32,
    position: Vec2,
    layer: usize,

    children: Option<Box<[Quadtree; 4]>>,
    items: Vec<QuadData>,
}

impl Quadtree {
    pub fn new(width: f32, height: f32, position: Vec2, layer: usize) -> Self {
        Self { width, height, position, layer, ..Default::default() }
    }
}

impl Default for Quadtree {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            position: Default::default(),
            layer: 0,
            children: None,
            items: vec![],
        }
    }
}

impl Quadtree {
    pub fn size(&self) -> usize {
        let mut count = self.items.len();

        if let Some(children) = &self.children {
            count += children[0].size();
            count += children[1].size();
            count += children[2].size();
            count += children[3].size();
        }

        count
    }

    pub fn get_max_layer_depth(&self) -> usize {
        let mut max_layer = self.layer;

        if let Some(children) = self.children.as_ref() {
            let new_layer = children[0].get_max_layer_depth();
            if new_layer > max_layer {
                max_layer = new_layer;
            }
            let new_layer = children[1].get_max_layer_depth();
            if new_layer > max_layer {
                max_layer = new_layer;
            }
            let new_layer = children[2].get_max_layer_depth();
            if new_layer > max_layer {
                max_layer = new_layer;
            }
            let new_layer = children[3].get_max_layer_depth();
            if new_layer > max_layer {
                max_layer = new_layer;
            }
        }

        max_layer
    }

    pub fn get_all_squares(&self, output: &mut Vec<(Vec2, f32, f32, usize)>) {
        if let Some(children) = self.children.as_ref() {
            children[0].get_all_squares(output);
            children[1].get_all_squares(output);
            children[2].get_all_squares(output);
            children[3].get_all_squares(output);
        } else {
            output.push((self.position, self.width, self.height, self.layer));
        }
    }

    pub fn query_entities(&self, output: &mut Vec<QuadData>, position: &Vec3, size: &Vec2) {
        if !self.overlap_rectangle(position, size) {
            return;
        }

        if self.is_contained_in(position, size) {
            self.get_entities(output)
        }

        output.extend(&self.items);
        if let Some(children) = &self.children {
            children[0].query_entities(output, position, size);
            children[1].query_entities(output, position, size);
            children[2].query_entities(output, position, size);
            children[3].query_entities(output, position, size);
        }
    }

    pub fn get_entities(&self, output: &mut Vec<QuadData>) {
        output.extend(&self.items);

        if let Some(children) = &self.children {
            children[0].get_entities(output);
            children[1].get_entities(output);
            children[2].get_entities(output);
            children[3].get_entities(output);
        }
    }

    pub fn insert(&mut self, data: &QuadData) -> bool {
        let size = match data.collider_type {
            Circle(radius) => Vec2::new(radius, radius),
            Rectangle(size) => size,
        };

        if !self.contains_rectangle(&data.position, &size) {
            return false;
        }

        if self.layer >= 15 {
            self.items.push(*data);
            return true;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(children) = self.children.as_mut() {
            if children[0].insert(data) {
                return true;
            };
            if children[1].insert(data) {
                return true;
            };
            if children[2].insert(data) {
                return true;
            };
            if children[3].insert(data) {
                return true;
            };
        }

        self.items.push(*data);
        true
    }

    fn subdivide(&mut self) {
        let new_width = self.width / 4.0;
        let new_height = self.height / 4.0;
        let new_width2 = self.height / 2.0;
        let new_height2 = self.height / 2.0;

        self.children = Some(Box::new([
            Self::new(new_width2, new_height2, Vec2::new(self.position.x - new_width, self.position.y + new_height), self.layer + 1),
            Self::new(new_width2, new_height2, Vec2::new(self.position.x + new_width, self.position.y + new_height), self.layer + 1),
            Self::new(new_width2, new_height2, Vec2::new(self.position.x - new_width, self.position.y - new_height), self.layer + 1),
            Self::new(new_width2, new_height2, Vec2::new(self.position.x + new_width, self.position.y - new_height), self.layer + 1)
        ]));
    }

    fn contains_rectangle(&self, position: &Vec3, size: &Vec2) -> bool {
        let a_min_x = self.position.x - self.width / 2.0;
        let a_min_y = self.position.y - self.height / 2.0;

        let a_max_x = self.position.x + self.width / 2.0;
        let a_max_y = self.position.y + self.height / 2.0;

        let b_min_x = position.x - size.x / 2.0;
        let b_min_y = position.y - size.y / 2.0;

        let b_max_x = position.x + size.x / 2.0;
        let b_max_y = position.y + size.y / 2.0;

        a_min_x <= b_min_x
            && a_max_x >= b_max_x
            && a_min_y <= b_min_y
            && a_max_y >= b_max_y
    }

    fn overlap_rectangle(&self, position: &Vec3, size: &Vec2) -> bool {
        let a_min_x = self.position.x - self.width / 2.0;
        let a_min_y = self.position.y - self.height / 2.0;

        let a_max_x = self.position.x + self.width / 2.0;
        let a_max_y = self.position.y + self.height / 2.0;

        let b_min_x = position.x - size.x / 2.0;
        let b_min_y = position.y - size.y / 2.0;

        let b_max_x = position.x + size.x / 2.0;
        let b_max_y = position.y + size.y / 2.0;

        a_min_x <= b_max_x
            && a_max_x >= b_min_x
            && a_min_y <= b_max_y
            && a_max_y >= b_min_y
    }

    fn is_contained_in(&self, position: &Vec3, size: &Vec2) -> bool {
        let b_min_x = self.position.x - self.width / 2.0;
        let b_min_y = self.position.y - self.height / 2.0;

        let b_max_x = self.position.x + self.width / 2.0;
        let b_max_y = self.position.y + self.height / 2.0;

        let a_min_x = position.x - size.x / 2.0;
        let a_min_y = position.y - size.y / 2.0;

        let a_max_x = position.x + size.x / 2.0;
        let a_max_y = position.y + size.y / 2.0;

        a_min_x <= b_min_x
            && a_max_x >= b_max_x
            && a_min_y <= b_min_y
            && a_max_y >= b_max_y
    }
}