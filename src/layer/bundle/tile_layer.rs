use bevy::prelude::*;

use crate::project::ProjectAsset;

#[derive(Clone, Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadTileLayerSettings {
    ComponentOnly,
    #[default]
    Mesh,
}

#[derive(Bundle, Debug, Default)]
pub(crate) struct TileLayerBundle {
    project: Handle<ProjectAsset>,
    settings: LoadTileLayerSettings,
}

