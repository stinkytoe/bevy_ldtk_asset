use bevy::prelude::*;

use crate::world::LevelsToLoad;
use crate::world::WorldAsset;

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub(crate) world: Handle<WorldAsset>,
    pub(crate) levels_to_load: LevelsToLoad,
    pub(crate) spatial: SpatialBundle,
}
