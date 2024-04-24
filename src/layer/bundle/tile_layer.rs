use bevy::prelude::*;

use crate::layer::LayerComponent;
use crate::project::ProjectAsset;

#[derive(Clone, Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadTileLayerSettings {
    ComponentOnly,
    #[default]
    Mesh,
}

#[derive(Bundle, Debug)]
pub(crate) struct TileLayerBundle {
    pub(crate) project: Handle<ProjectAsset>,
    pub(crate) layer: LayerComponent,
    pub(crate) settings: LoadTileLayerSettings,
}
