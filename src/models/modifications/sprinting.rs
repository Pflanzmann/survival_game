use bevy::ecs::component::Component;

#[derive(Component, Copy, Clone)]
pub struct Sprinting {
    pub boost_amount: f32,
}
