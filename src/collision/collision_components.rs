use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Collision {
    pub collision_entities: Vec<Box<Collider>>,
}