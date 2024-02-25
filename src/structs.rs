use bevy::prelude::*;

#[derive(Component, Default)]
pub enum LoadParameters {
    #[default]
    Nothing,
    Everything,
}
