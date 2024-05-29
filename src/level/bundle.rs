use bevy::prelude::*;

use crate::layer::LayersToLoad;
use crate::level::LevelAsset;

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub(crate) level: Handle<LevelAsset>,
    pub(crate) layers_to_load: LayersToLoad,
    pub(crate) spatial: SpatialBundle,
}
