use bevy::prelude::*;

use crate::{assets::world::WorldAsset, structs::LoadParameters};

#[derive(Bundle, Default)]
pub struct WorldBundle {
    pub world: Handle<WorldAsset>,
    pub load_parameters: LoadParameters,
}
