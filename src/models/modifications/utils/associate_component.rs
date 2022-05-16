use bevy::prelude::Component;

pub trait AssociateComponent<T: Component> {
    fn get_component(&self) -> T;
}