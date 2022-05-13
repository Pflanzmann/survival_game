use bevy::prelude::Vec2;

#[derive(Copy, Clone)]
pub struct QuadData<T> where T: Copy + Clone {
    pub position: Vec2,
    pub size: Vec2,
    pub data: T,
}

pub struct Quadtree<T> where T: Copy + Clone {
    width: f32,
    height: f32,
    position: Vec2,
    layer: usize,

    children: Option<Box<[Quadtree<T>; 4]>>,
    items: Vec<QuadData<T>>,
}

impl<T> Quadtree<T> where T: Copy + Clone {
    pub fn new(width: f32, height: f32, position: Vec2, layer: usize) -> Self {
        Self { width, height, position, layer, ..Default::default() }
    }
}

impl<T> Default for Quadtree<T> where T: Copy + Clone {
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

impl<T> Quadtree<T> where T: Copy + Clone {
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

    pub fn query_entities(&self, output: &mut Vec<QuadData<T>>, position: &Vec2, size: &Vec2) {
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

    pub fn get_entities(&self, output: &mut Vec<QuadData<T>>) {
        output.extend(&self.items);

        if let Some(children) = &self.children {
            children[0].get_entities(output);
            children[1].get_entities(output);
            children[2].get_entities(output);
            children[3].get_entities(output);
        }
    }

    pub fn insert(&mut self, quad_data: &QuadData<T>) -> bool {
        if !self.contains_rectangle(&quad_data.position, &quad_data.size) {
            return false;
        }

        if self.layer >= 15 {
            self.items.push(*quad_data);
            return true;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(children) = self.children.as_mut() {
            if children[0].insert(quad_data) {
                return true;
            };
            if children[1].insert(quad_data) {
                return true;
            };
            if children[2].insert(quad_data) {
                return true;
            };
            if children[3].insert(quad_data) {
                return true;
            };
        }

        self.items.push(*quad_data);
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

    fn contains_rectangle(&self, position: &Vec2, size: &Vec2) -> bool {
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

    fn overlap_rectangle(&self, position: &Vec2, size: &Vec2) -> bool {
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

    fn is_contained_in(&self, position: &Vec2, size: &Vec2) -> bool {
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