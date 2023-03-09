use bevy::prelude::{Entity, Resource};

#[derive(Default, Resource)]
pub struct ShopCustomer {
    pub customer: Option<Entity>,
}