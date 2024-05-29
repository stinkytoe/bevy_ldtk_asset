use bevy::prelude::*;

use crate::entity::EntitiesToLoad;
use crate::layer::LayerAsset;

#[derive(Bundle, Debug, Default)]
pub struct LayerBundle {
    pub(crate) layer: Handle<LayerAsset>,
    pub(crate) entities_to_load: EntitiesToLoad,
    pub(crate) spatial: SpatialBundle,
}
