use std::collections::LinkedList;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy::prelude::{Entity, Vec2};
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(calc_tree)
        .add_system(camer_controle)
        .add_system(collision_detection)
        .add_system(naive_collision_detection)
        .add_system(reset)
        .init_resource::<TreeHolder>()
        .run()
}

#[derive(Component)]
pub struct UnitSize(Vec2);

#[derive(Component)]
pub struct TreeArea;

#[derive(Default)]
pub struct TreeHolder(Quadtree);

const SPAWN_AMOUNT: usize = 30000;
const SPAWN_AREA_WIDTH: f32 = 30000.0;
const SPAWN_AREA_HEIGHT: f32 = 30000.0;
const UNIT_WIDTH: f32 = 100.0;
const UNIT_HEIGHT: f32 = 100.0;
const UNIT_SIZE_MIN: f32 = 10.0;


const TREE_HEIGHT: f32 = SPAWN_AREA_WIDTH;
const TREE_WIDTH: f32 = SPAWN_AREA_HEIGHT;
const TREE_MAX_LAYER: usize = 10;

fn collision_detection(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &UnitSize)>,
    quad_tree: Res<TreeHolder>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Numpad2) {
        let start_time = SystemTime::now();

        for (entity, _, _) in entity_query.iter() {
            commands.entity(entity).insert(Visibility { is_visible: false });
        }
        let mut counter = 0;
        let mut biggest_comparator = 0;

        for (entity, trans, size_a) in entity_query.iter() {
            let mut check_list: Vec<Entity> = Vec::new();
            quad_tree.0.query_entities(
                &mut check_list,
                &trans.translation.truncate(),
                &size_a.0,
            );

            if check_list.len() > biggest_comparator {
                biggest_comparator = check_list.len();
            }

            for other_entity in check_list.iter() {
                if *other_entity == entity { continue; }

                let (_, other_transform, other_size) = match entity_query.get(*other_entity) {
                    Ok(value) => value,
                    Err(_) => { continue; }
                };

                if is_colliding(trans.translation, size_a.0, other_transform.translation, other_size.0) {
                    commands.entity(entity).insert(Visibility { is_visible: true });
                }

                counter += 1;
            }
        }

        let end = SystemTime::now();
        println!("new system time: {:?} | counter = {counter} | biggest_comp: {biggest_comparator}", end.duration_since(start_time));
    }

    if input.just_pressed(KeyCode::Numpad0) {
        for (e, _, _) in entity_query.iter() {
            commands.entity(e).insert(Visibility { is_visible: true });
        }
    }

    if input.just_pressed(KeyCode::Numpad4) {
        for (e, _, _) in entity_query.iter() {
            commands.entity(e).insert(Visibility { is_visible: false });
        }
    }
}

fn naive_collision_detection(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &UnitSize), Without<Camera>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Numpad3) {
        let mut counter = 0;
        let start_time = SystemTime::now();

        for (entity, _, _) in entity_query.iter() {
            commands.entity(entity).insert(Visibility { is_visible: false });
        }

        for (entity, trans, size) in entity_query.iter() {
            for (other_entity, other_trans, other_size) in entity_query.iter() {
                if entity == other_entity { continue; }
                if is_colliding(trans.translation, size.0, other_trans.translation, other_size.0) {
                    commands.entity(entity).insert(Visibility { is_visible: true });
                }

                counter += 1;
            }
        }
        let end = SystemTime::now();
        println!("old system time: {:?} | counter = {}", end.duration_since(start_time), counter);
    }
}

fn calc_tree(
    mut commands: Commands,
    area_query: Query<Entity, (With<TreeArea>, Without<UnitSize>, Without<OrthographicProjection>)>,
    entity_query: Query<(Entity, &Transform, &UnitSize), (Without<TreeArea>, Without<Camera>)>,
    mut quad_tree: ResMut<TreeHolder>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Numpad1) {
        let mut counter = 0;
        let mut failed_counter = 0;
        let init_layer = 0;

        quad_tree.0 = Quadtree::new(TREE_WIDTH, TREE_HEIGHT, Vec2::new(TREE_WIDTH / 2.0, TREE_HEIGHT / 2.0), init_layer);

        let start_time = SystemTime::now();
        for (entity, transform, size) in entity_query.iter() {
            if !quad_tree.0.insert(&entity, &transform.translation.truncate(), &size.0) {
                failed_counter += 1;
            };
            counter += 1;
        }

        let end = SystemTime::now();
        println!("calc tree time: {:?} | contains: {} | counter: {} | failed: {} | init_layer: {} | layer: {}",
                 end.duration_since(start_time),
                 quad_tree.0.size(),
                 counter,
                 failed_counter,
                 init_layer,
                 quad_tree.0.print_structure()
        );
        for entity in area_query.iter() {
            commands.entity(entity).despawn();
        }

        let mut output: Vec<(Vec2, f32, f32, usize)> = Vec::new();
        quad_tree.0.get_all_squares(&mut output);

        for (pos, width, height, layer) in output {
            let color = 0.1 * layer as f32;
            commands.spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::from([color, color, color, color]),
                        custom_size: Some(Vec2::new(width * 0.9, height * 0.9)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    ..Default::default()
                }
            ).insert(TreeArea);
        }
    }
    if input.just_pressed(KeyCode::Numpad5) {
        for entity in area_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn camer_controle(
    input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut proj) in camera_query.iter_mut() {
        if input.pressed(KeyCode::W) {
            transform.translation += Vec3::new(0.0, 40.0, 0.0);
        }

        if input.pressed(KeyCode::S) {
            transform.translation += Vec3::new(0.0, -40.0, 0.0);
        }

        if input.pressed(KeyCode::A) {
            transform.translation += Vec3::new(-40.0, 0.0, 0.0);
        }

        if input.pressed(KeyCode::D) {
            transform.translation += Vec3::new(40.0, 0.0, 0.0);
        }

        if input.pressed(KeyCode::E) {
            proj.scale += 1.0;
        }

        if input.pressed(KeyCode::Q) {
            proj.scale += -1.0;
        }
    }
}

fn reset(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    entity_query: Query<(Entity, &Transform), Without<Camera>>,
) {
    if input.just_pressed(KeyCode::Numpad6) {
        for entity in entity_query.iter() {
            commands.entity(entity.0).despawn();
        }

        for _ in 0..SPAWN_AMOUNT {
            let random_x: f32 = random::<f32>() * SPAWN_AREA_WIDTH;
            let random_y: f32 = random::<f32>() * SPAWN_AREA_HEIGHT;

            let random_size_a: f32 = (random::<f32>() * UNIT_WIDTH) + UNIT_SIZE_MIN;
            let random_size_b: f32 = (random::<f32>() * UNIT_HEIGHT) + UNIT_SIZE_MIN;

            let random_r: f32 = random::<f32>();
            let random_g: f32 = random::<f32>();
            let random_b: f32 = random::<f32>();


            commands.spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(random_size_a, random_size_b)),
                        color: Color::from([random_r, random_g, random_b]),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    ..Default::default()
                }
            ).insert(UnitSize(Vec2::new(random_size_a, random_size_b)));
        }
    }
}

fn setup_system(
    mut commands: Commands,
) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation.x = TREE_WIDTH / 2.0;
    camera_bundle.transform.translation.y = TREE_HEIGHT / 2.0;
    camera_bundle.transform.translation.z = 10.0;
    camera_bundle.orthographic_projection.scale = 10.0;

    commands.spawn_bundle(camera_bundle);

    for _ in 0..SPAWN_AMOUNT {
        let random_x: f32 = random::<f32>() * SPAWN_AREA_WIDTH;
        let random_y: f32 = random::<f32>() * SPAWN_AREA_HEIGHT;

        let random_size_a: f32 = (random::<f32>() * UNIT_WIDTH) + UNIT_SIZE_MIN;
        let random_size_b: f32 = (random::<f32>() * UNIT_HEIGHT) + UNIT_SIZE_MIN;

        let random_r: f32 = random::<f32>();
        let random_g: f32 = random::<f32>();
        let random_b: f32 = random::<f32>();


        commands.spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(random_size_a, random_size_b)),
                    color: Color::from([random_r, random_g, random_b]),
                    ..Default::default()
                },
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..Default::default()
            }
        ).insert(UnitSize(Vec2::new(random_size_a, random_size_b)));
    }
}

#[derive(Default)]
pub struct Quadtree {
    width: f32,
    height: f32,
    position: Vec2,
    layer: usize,

    children: Option<Box<[Quadtree; 4]>>,
    items: Vec<Entity>,
}

impl Quadtree {
    pub fn new(width: f32, height: f32, position: Vec2, layer: usize) -> Self {
        Self { width, height, position, layer, ..Default::default() }
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

    pub fn print_structure(&self) -> usize {
        let mut max_layer = self.layer;

        if let Some(children) = self.children.as_ref() {
            let new_layer = children[0].print_structure();
            if new_layer > max_layer {
                max_layer = new_layer;
            }
            let new_layer = children[1].print_structure();
            if new_layer > max_layer {
                max_layer = new_layer;
            }
            let new_layer = children[2].print_structure();
            if new_layer > max_layer {
                max_layer = new_layer;
            }
            let new_layer = children[3].print_structure();
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

    pub fn query_entities(&self, output: &mut Vec<Entity>, position: &Vec2, size: &Vec2) {
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

    pub fn get_entities(&self, output: &mut Vec<Entity>) {
        output.extend(&self.items);

        if let Some(children) = &self.children {
            children[0].get_entities(output);
            children[1].get_entities(output);
            children[2].get_entities(output);
            children[3].get_entities(output);
        }
    }

    pub fn insert(&mut self, data: &Entity, position: &Vec2, size: &Vec2) -> bool {
        if !self.contains_rectangle(position, size) {
            return false;
        }

        if self.layer >= TREE_MAX_LAYER {
            self.items.push(*data);
            return true;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(children) = self.children.as_mut() {
            if children[0].insert(data, position, size) {
                return true;
            };
            if children[1].insert(data, position, size) {
                return true;
            };
            if children[2].insert(data, position, size) {
                return true;
            };
            if children[3].insert(data, position, size) {
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

pub fn is_colliding(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    return a_min.x <= b_max.x
        && a_max.x >= b_min.x
        && a_min.y <= b_max.y
        && a_max.y >= b_min.y;
}