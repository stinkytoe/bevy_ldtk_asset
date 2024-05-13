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
    pub load_layers: LoadLayers,
    pub load_tile_layer_settings: LoadTileLayerSettings,
    pub load_entities: LoadEntities,
    pub load_entity_layer_settings: LoadEntityLayerSettings,
    pub level_separation: f32,
    pub layer_separation: f32,
}

impl Default for LevelBundleLoadSettings {
    fn default() -> Self {
        Self {
            load_bg_color: true,
            load_bg_image: true,
            load_layers: Default::default(),
            load_tile_layer_settings: Default::default(),
            load_entities: Default::default(),
            load_entity_layer_settings: Default::default(),
            level_separation: 10.0,
            layer_separation: 0.1,
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub level: Handle<LevelAsset>,
    pub load_settings: LevelBundleLoadSettings,
    pub spatial: SpatialBundle,
}
