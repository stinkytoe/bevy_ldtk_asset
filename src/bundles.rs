use bevy::prelude::*;

use crate::assets::world::WorldAsset;

#[derive(Component, Default)]
pub enum LoadParameters {
    #[default]
    LoadNothing,
}

#[derive(Bundle, Default)]
pub struct WorldBundle {
    pub world: Handle<WorldAsset>,
    pub load_parameters: LoadParameters,
}
