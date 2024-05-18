use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::layer::LayersToLoad;
use crate::level::LevelAsset;

#[derive(Clone, Component, Debug, Default, Reflect)]
pub enum LevelsToLoad {
    None,
    ByIdentifiers(HashMap<String, LayersToLoad>),
    ByIids(HashMap<String, LayersToLoad>),
    #[default]
    All,
}

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub(crate) level: Handle<LevelAsset>,
    pub(crate) layers_to_load: LayersToLoad,
    pub(crate) spatial: SpatialBundle,
}
