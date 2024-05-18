use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::entity::EntitiesToLoad;
use crate::layer::LayerAsset;

#[derive(Clone, Component, Debug, Default, Reflect)]
pub enum LayersToLoad {
    None,
    ByIdentifiers(HashMap<String, EntitiesToLoad>),
    ByIids(HashMap<String, EntitiesToLoad>),
    TileLayersOnly,
    EntityLayersOnly,
    #[default]
    All,
}

#[derive(Bundle, Debug, Default)]
pub struct LayerBundle {
    pub(crate) layer: Handle<LayerAsset>,
    pub(crate) entities_to_load: EntitiesToLoad,
    pub(crate) spatial: SpatialBundle,
}
