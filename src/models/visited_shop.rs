use bevy::ecs::component::Component;

#[derive(Component)]
pub struct VisitedShop {
    pub revisit_timer: f32,
}

impl VisitedShop {
    pub fn new(revisit_timer: f32) -> Self {
        VisitedShop { revisit_timer }
    }
}
