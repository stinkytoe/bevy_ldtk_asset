use bevy::prelude::*;

use crate::layer::LayerComponent;
use crate::project::ProjectAsset;

#[derive(Clone, Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadTileLayerMeshSettings {
    ComponentOnly,
    #[default]
    Mesh,
}

#[derive(Clone, Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LoadTileLayerSettings {
    pub load_int_grids: bool,
    pub mesh_settings: LoadTileLayerMeshSettings,
}

impl Default for LoadTileLayerSettings {
    fn default() -> Self {
        Self {
            load_int_grids: true,
            mesh_settings: Default::default(),
        }
    }
}

#[derive(Bundle, Debug)]
pub(crate) struct TileLayerBundle {
    pub(crate) project: Handle<ProjectAsset>,
    pub(crate) layer: LayerComponent,
    pub(crate) settings: LoadTileLayerSettings,
    pub(crate) spatial: SpatialBundle,
}
