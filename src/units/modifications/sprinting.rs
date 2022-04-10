use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Sprinting {
    pub boost_amount: f32,
}
