use bevy::prelude::*;

use crate::layer::LoadEntities;
use crate::layer::LoadEntityLayerSettings;
use crate::layer::LoadTileLayerSettings;
use crate::level::LevelAsset;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadLayers {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    TileLayers,
    EntityLayers,
    #[default]
    All,
}

#[derive(Clone, Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LevelBundleLoadSettings {
    pub load_bg_color: bool,
    pub load_bg_image: bool,
    pub load_int_grids: bool,
    pub load_layers: LoadLayers,
    pub load_tile_layer_settings: LoadTileLayerSettings,
    pub load_entities: LoadEntities,
    pub load_entity_layer_settings: LoadEntityLayerSettings,
}

impl Default for LevelBundleLoadSettings {
    fn default() -> Self {
        Self {
            load_bg_color: true,
            load_bg_image: true,
            load_int_grids: true,
            load_layers: Default::default(),
            load_tile_layer_settings: Default::default(),
            load_entities: Default::default(),
            load_entity_layer_settings: Default::default(),
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub level: Handle<LevelAsset>,
    pub load_settings: LevelBundleLoadSettings,
    pub spatial: SpatialBundle,
}
