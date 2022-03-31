use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CurveShot {
    pub curve_left: bool,
}