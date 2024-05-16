use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::level::LevelsToLoad;
use crate::world::WorldAsset;

#[derive(Component, Debug, Default, Reflect)]
pub enum WorldsToLoad {
    None,
    ByIdentifiers(HashMap<String, LevelsToLoad>),
    ByIids(HashMap<String, LevelsToLoad>),
    #[default]
    All,
}

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub(crate) world: Handle<WorldAsset>,
    pub(crate) levels_to_load: LevelsToLoad,
    pub(crate) spatial: SpatialBundle,
}
